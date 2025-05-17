use crate::{
    Error,
    db::{
        DbLocal,
        types::{DataRow, EntityRow, EntityValue, SortKey},
    },
    query::{
        DebugContext, QueryError, Resolver,
        load::{LoadError, LoadFormat, LoadMap, LoadMethod, LoadResponse, Loader},
        types::{Filter, Order},
    },
    traits::Entity,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

///
/// LoadBuilder
///

#[derive(Debug, Default)]
pub struct LoadBuilder<E>
where
    E: Entity,
{
    phantom: PhantomData<E>,
}

impl<E> LoadBuilder<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // method
    #[must_use]
    pub fn method(self, method: LoadMethod) -> LoadQuery {
        LoadQuery::new(E::PATH, method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQuery {
        LoadQuery::new(E::PATH, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQuery {
        LoadQuery::new(E::PATH, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQuery {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQuery::new(E::PATH, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQuery {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQuery::new(E::PATH, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQuery {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        LoadQuery::new(E::PATH, method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQuery {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        LoadQuery::new(E::PATH, method)
    }
}

///
/// LoadQuery
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct LoadQuery {
    pub path: String,
    pub method: LoadMethod,
    pub format: LoadFormat,
    pub offset: u32,
    pub limit: Option<u32>,
    pub filter: Option<Filter>,
    pub order: Option<Order>,
    pub debug: DebugContext,
}

impl LoadQuery {
    // new
    #[must_use]
    pub fn new(path: &str, method: LoadMethod) -> Self {
        Self {
            path: path.to_string(),
            method,
            format: LoadFormat::default(),
            offset: 0,
            limit: None,
            filter: None,
            order: None,
            debug: DebugContext::default(),
        }
    }

    // format
    #[must_use]
    pub const fn format(mut self, format: LoadFormat) -> Self {
        self.format = format;
        self
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // offset
    #[must_use]
    pub const fn offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    // limit
    #[must_use]
    pub const fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    // limit_option
    #[must_use]
    pub const fn limit_option(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;
        self
    }

    // filter
    #[must_use]
    pub fn filter<T: Into<Filter>>(mut self, filter: T) -> Self {
        self.filter = Some(filter.into());
        self
    }

    // filter_option
    #[must_use]
    pub fn filter_option(mut self, filter: Option<Filter>) -> Self {
        self.filter = filter;
        self
    }

    // filter_all
    #[must_use]
    pub fn filter_all(mut self, text: &str) -> Self {
        self.filter = Some(Filter::all(text.to_string()));
        self
    }

    // filter_fields
    #[must_use]
    pub fn filter_fields(mut self, fields: &[(String, String)]) -> Self {
        self.filter = Some(Filter::fields(fields.into()));
        self
    }

    // order
    #[must_use]
    pub fn order<T: Into<Order>>(mut self, order: T) -> Self {
        self.order = Some(order.into());
        self
    }

    // order_option
    #[must_use]
    pub fn order_option<T: Into<Order>>(mut self, order: Option<T>) -> Self {
        self.order = order.map(Into::into);
        self
    }

    // execute
    // excutes the query and returns a collection
    pub fn execute<E>(self, db: DbLocal) -> Result<LoadCollection<E>, Error>
    where
        E: Entity,
    {
        let executor = LoadExecutor::<E>::new(self);
        executor.execute(db)
    }

    // response
    pub fn response<E>(self, db: DbLocal) -> Result<LoadResponse, Error>
    where
        E: Entity,
    {
        let executor = LoadExecutor::<E>::new(self);
        executor.response(db)
    }
}

///
/// LoadExecutor
///

pub struct LoadExecutor<E>
where
    E: Entity,
{
    query: LoadQuery,
    phantom: PhantomData<E>,
}

impl<E> LoadExecutor<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub const fn new(query: LoadQuery) -> Self {
        Self {
            query,
            phantom: PhantomData,
        }
    }

    // execute
    pub fn execute(self, db: DbLocal) -> Result<LoadCollection<E>, Error> {
        // loader
        let resolver = Resolver::new(&self.query.path);
        let loader = Loader::new(db, resolver);
        let res = loader.load(&self.query.method)?;

        // convert
        let rows = res
            .into_iter()
            .filter(|row| row.value.path == E::path())
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()
            .map_err(QueryError::SerializeError)?;

        // filter
        let rows = rows
            .into_iter()
            .filter(|row| match &self.query.filter {
                Some(Filter::All(text)) => row.value.entity.filter_all(text),
                Some(Filter::Fields(fields)) => row.value.entity.filter_fields(fields.clone()),
                None => true,
            })
            .collect::<Vec<_>>();

        // sort
        let mut rows = rows;
        if let Some(order) = &self.query.order {
            let sorter = E::sort(order);
            rows.sort_by(|a, b| sorter(&a.value.entity, &b.value.entity));
        }

        // offset and limit
        let rows = rows
            .into_iter()
            .skip(self.query.offset as usize)
            .take(self.query.limit.unwrap_or(u32::MAX) as usize)
            .collect();

        Ok(LoadCollection(rows))
    }

    // response
    pub fn response(self, db: DbLocal) -> Result<LoadResponse, Error> {
        let format = self.query.format.clone();
        let collection = self.execute(db)?;

        let response = match format {
            LoadFormat::Rows => LoadResponse::Rows(collection.data_rows()?),
            LoadFormat::Keys => LoadResponse::Keys(collection.keys()),
            LoadFormat::Count => LoadResponse::Count(collection.count()),
        };

        Ok(response)
    }
}

///
/// LoadCollection
///

#[derive(Debug)]
pub struct LoadCollection<E: Entity>(pub Vec<EntityRow<E>>);

impl<E> LoadCollection<E>
where
    E: Entity,
{
    // count
    #[must_use]
    pub fn count(self) -> usize {
        self.0.len()
    }

    // map
    #[must_use]
    pub fn map(self) -> LoadMap<EntityValue<E>> {
        let pairs = self
            .0
            .into_iter()
            .map(|row| (row.key.into(), row.value))
            .collect::<Vec<_>>();

        LoadMap::from_pairs(pairs)
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<SortKey> {
        self.0.first().map(|row| row.key.clone())
    }

    // try_key
    pub fn try_key(self) -> Result<SortKey, QueryError> {
        let row = self.0.first().ok_or(LoadError::NoResultsFound)?;

        Ok(row.key.clone())
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<SortKey> {
        self.0.into_iter().map(|row| row.key).collect()
    }

    // data_rows
    pub fn data_rows(self) -> Result<Vec<DataRow>, QueryError> {
        self.0
            .into_iter()
            .map(|row| row.try_into().map_err(QueryError::SerializeError))
            .collect()
    }

    // entity
    #[must_use]
    pub fn entity(self) -> Option<E> {
        self.0.first().map(|row| row.value.entity.clone())
    }

    // try_entity
    pub fn try_entity(self) -> Result<E, QueryError> {
        let res = self
            .0
            .first()
            .map(|row| row.value.entity.clone())
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(res)
    }

    // entities
    #[must_use]
    pub fn entities(self) -> Vec<E> {
        self.0.iter().map(|row| row.value.entity.clone()).collect()
    }

    // entity_row
    #[must_use]
    pub fn entity_row(self) -> Option<EntityRow<E>> {
        self.0.first().cloned()
    }

    // try_entity_row
    pub fn try_entity_row(self) -> Result<EntityRow<E>, QueryError> {
        let res = self
            .0
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
}
