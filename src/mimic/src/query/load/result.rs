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

///
/// LoadResult
///

pub struct LoadResult<E>
where
    E: Entity,
{
    rows: Vec<EntityRow<E>>,
}

impl<E> LoadResult<E>
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

        Self { rows }
    }

    // count
    #[must_use]
    pub fn count(self) -> usize {
        self.rows.len()
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<DataKey> {
        self.rows.first().map(|row| row.key.clone())
    }

    // try_key
    pub fn try_key(self) -> Result<DataKey, Error> {
        let row = self
            .rows
            .first()
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(row.key.clone())
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<DataKey> {
        self.rows.into_iter().map(|row| row.key).collect()
    }

    // entity
    #[must_use]
    pub fn entity(self) -> Option<E> {
        self.rows.first().map(|row| row.value.entity.clone())
    }

    // try_entity
    pub fn try_entity(self) -> Result<E, Error> {
        let res = self
            .rows
            .first()
            .map(|row| row.value.entity.clone())
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(res)
    }

    // entities
    #[must_use]
    pub fn entities(self) -> Vec<E> {
        self.rows.into_iter().map(|row| row.value.entity).collect()
    }

    // entity_row
    #[must_use]
    pub fn entity_row(self) -> Option<EntityRow<E>> {
        self.rows.first().cloned()
    }

    // try_entity_row
    pub fn try_entity_row(self) -> Result<EntityRow<E>, Error> {
        let res = self
            .rows
            .first()
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(res.clone())
    }

    // entity_rows
    #[must_use]
    pub fn entity_rows(self) -> Vec<EntityRow<E>> {
        self.rows
    }
}

///
/// LoadResultDyn
///
/// Complex logic is handled better with iter::from_fn and move_next().
/// All iterator methods (for now) are consuming.
///

#[derive(Debug)]
pub struct LoadResultDyn {
    rows: Vec<DataRow>,
}

impl LoadResultDyn {
    #[must_use]
    pub fn new(rows: Vec<DataRow>, limit: Option<u32>, offset: u32) -> Self {
        let rows = apply_offset_limit(rows, offset, limit);

        Self { rows }
    }

    // count
    #[must_use]
    pub fn count(self) -> usize {
        self.rows.len()
    }

    // data_row
    #[must_use]
    pub fn data_row(self) -> Option<DataRow> {
        self.rows.first().cloned()
    }

    // data_rows
    #[must_use]
    pub fn data_rows(self) -> Vec<DataRow> {
        self.rows
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<DataKey> {
        self.rows.first().map(|row| row.key.clone())
    }

    // try_key
    pub fn try_key(self) -> Result<DataKey, Error> {
        let row = self
            .rows
            .first()
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(row.key.clone())
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<DataKey> {
        self.rows.into_iter().map(|row| row.key).collect()
    }

    // try_blob
    pub fn try_blob(self) -> Result<Vec<u8>, Error> {
        self.rows
            .into_iter()
            .next()
            .map(|row| row.value.data)
            .ok_or_else(|| QueryError::LoadError(LoadError::NoResultsFound).into())
    }

    // blobs
    #[must_use]
    pub fn blobs(self) -> Vec<Vec<u8>> {
        self.rows.into_iter().map(|row| row.value.data).collect()
    }
}

// apply_offset_limit
fn apply_offset_limit<T>(rows: Vec<T>, offset: u32, limit: Option<u32>) -> Vec<T> {
    rows.into_iter()
        .skip(offset as usize)
        .take(limit.unwrap_or(u32::MAX) as usize)
        .collect()
}
