use crate::{
    Error,
    db::types::{DataKey, DataRow, EntityRow},
    orm::traits::Entity,
    query::{
        QueryError,
        load::LoadError,
        types::{Filter, Order},
    },
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};

///
/// LoadResponse
///

#[derive(Debug, Deref, DerefMut, IntoIterator)]
pub struct LoadResponse<E: Entity>(Vec<EntityRow<E>>);

impl<E> LoadResponse<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new(
        rows: Vec<EntityRow<E>>,
        limit: Option<u32>,
        offset: u32,
        filter: Option<Filter>,
        order: Option<Order>,
    ) -> Self {
        // filter
        let rows = rows
            .into_iter()
            .filter(|row| match &filter {
                Some(Filter::All(text)) => row.value.entity.filter_all(text),
                Some(Filter::Fields(fields)) => row.value.entity.filter_fields(fields.clone()),
                None => true,
            })
            .collect::<Vec<_>>();

        // sort
        let mut rows = rows;
        if let Some(order) = order {
            let sorter = E::sort(&order);
            rows.sort_by(|a, b| sorter(&a.value.entity, &b.value.entity));
        }

        // offset and limit
        let rows = apply_offset_limit(rows, offset, limit);

        Self(rows)
    }

    // count
    #[must_use]
    pub fn count(self) -> usize {
        self.len()
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<DataKey> {
        self.first().map(|row| row.key.clone())
    }

    // try_key
    pub fn try_key(self) -> Result<DataKey, Error> {
        let row = self
            .first()
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(row.key.clone())
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<DataKey> {
        self.into_iter().map(|row| row.key).collect()
    }

    // entity
    #[must_use]
    pub fn entity(self) -> Option<E> {
        self.first().map(|row| row.value.entity.clone())
    }

    // try_entity
    pub fn try_entity(self) -> Result<E, Error> {
        let res = self
            .first()
            .map(|row| row.value.entity.clone())
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(res)
    }

    // entities
    #[must_use]
    pub fn entities(self) -> Vec<E> {
        self.into_iter().map(|row| row.value.entity).collect()
    }

    // entity_row
    #[must_use]
    pub fn entity_row(self) -> Option<EntityRow<E>> {
        self.first().cloned()
    }

    // try_entity_row
    pub fn try_entity_row(self) -> Result<EntityRow<E>, Error> {
        let res = self
            .first()
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(res.clone())
    }

    // entity_rows
    #[must_use]
    pub fn entity_rows(self) -> Vec<EntityRow<E>> {
        self.0
    }

    // as_dynamic
    pub fn as_dynamic(self) -> Result<LoadResponseDyn, Error> {
        let rows = self
            .into_iter()
            .map(|row| {
                let data_row: DataRow = row
                    .try_into()
                    .map_err(LoadError::SerializeError)
                    .map_err(QueryError::LoadError)?;

                Ok(data_row)
            })
            .collect::<Result<Vec<DataRow>, Error>>()?;

        Ok(LoadResponseDyn(rows))
    }
}

///
/// LoadResponseDyn
///

#[derive(CandidType, Debug, Deref, DerefMut, IntoIterator, Serialize, Deserialize)]
pub struct LoadResponseDyn(Vec<DataRow>);

impl LoadResponseDyn {
    #[must_use]
    pub fn new(rows: Vec<DataRow>, limit: Option<u32>, offset: u32) -> Self {
        let rows = apply_offset_limit(rows, offset, limit);

        Self(rows)
    }

    // count
    #[must_use]
    pub fn count(self) -> usize {
        self.len()
    }

    // data_row
    #[must_use]
    pub fn data_row(self) -> Option<DataRow> {
        self.first().cloned()
    }

    // data_rows
    #[must_use]
    pub fn data_rows(self) -> Vec<DataRow> {
        self.0
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<DataKey> {
        self.first().map(|row| row.key.clone())
    }

    // try_key
    pub fn try_key(self) -> Result<DataKey, Error> {
        let row = self
            .first()
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(row.key.clone())
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<DataKey> {
        self.into_iter().map(|row| row.key).collect()
    }

    // try_blob
    pub fn try_blob(self) -> Result<Vec<u8>, Error> {
        self.into_iter()
            .next()
            .map(|row| row.value.data)
            .ok_or_else(|| QueryError::LoadError(LoadError::NoResultsFound).into())
    }

    // blob
    #[must_use]
    pub fn blob(self) -> Option<Vec<u8>> {
        self.first().map(|row| row.value.data.clone())
    }

    // blobs
    #[must_use]
    pub fn blobs(self) -> Vec<Vec<u8>> {
        self.into_iter().map(|row| row.value.data).collect()
    }

    // as_generic
    // Converts LoadResponseDyn (Vec<DataRow>) into LoadResponse<E> (Vec<EntityRow<E>>)
    pub fn as_generic<E: Entity>(self) -> Result<LoadResponse<E>, Error> {
        let entity_rows = self
            .into_iter()
            .map(|row| {
                let entity_row: EntityRow<E> = row
                    .try_into()
                    .map_err(LoadError::SerializeError)
                    .map_err(QueryError::LoadError)?;

                Ok(entity_row)
            })
            .collect::<Result<Vec<EntityRow<E>>, Error>>()?;

        Ok(LoadResponse(entity_rows))
    }
}

// apply_offset_limit
fn apply_offset_limit<T>(rows: Vec<T>, offset: u32, limit: Option<u32>) -> Vec<T> {
    rows.into_iter()
        .skip(offset as usize)
        .take(limit.unwrap_or(u32::MAX) as usize)
        .collect()
}
