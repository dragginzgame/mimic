use crate::{
    Error,
    db::{
        DbLocal,
        types::{DataRow, EntityRow, EntityValue, SortKey},
    },
    query::{
        DebugContext, QueryError, Resolver,
        load::{
            LoadCollectionDyn, LoadError, LoadFormat, LoadMap, LoadMethod, LoadResponse, Loader,
        },
        traits::{LoadCollectionTrait, LoadQueryBuilderTrait},
        types::Order,
    },
    traits::Entity,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

///
/// LoadQuery
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoadQuery {
    pub method: LoadMethod,
    pub format: LoadFormat,
    pub offset: u32,
    pub limit: Option<u32>,
    pub search: Vec<(String, String)>,
    pub order: Option<Order>,
}

impl LoadQuery {
    #[must_use]
    pub fn new(method: LoadMethod) -> Self {
        Self {
            method,
            ..Default::default()
        }
    }
}

///
/// LoadQueryInit
///

#[derive(Debug, Default)]
pub struct LoadQueryInit<E>
where
    E: Entity,
{
    phantom: PhantomData<E>,
}

impl<E> LoadQueryInit<E>
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
    pub fn method(self, method: LoadMethod) -> LoadQueryBuilder<E> {
        LoadQueryBuilder::from_method(method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryBuilder<E> {
        LoadQueryBuilder::from_method(LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryBuilder<E> {
        LoadQueryBuilder::from_method(LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQueryBuilder<E> {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQueryBuilder::from_method(method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQueryBuilder<E> {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQueryBuilder::from_method(method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQueryBuilder<E> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        LoadQueryBuilder::from_method(method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQueryBuilder<E> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        LoadQueryBuilder::from_method(method)
    }
}

///
/// LoadQueryBuilder
///

#[allow(clippy::type_complexity)]
#[derive(Default)]
pub struct LoadQueryBuilder<E>
where
    E: Entity,
{
    query: LoadQuery,
    filters: Vec<Box<dyn Fn(&E) -> bool>>,
    debug: DebugContext,
}

impl<E: Entity> LoadQueryBuilder<E> {
    // new
    #[must_use]
    pub fn new(query: LoadQuery) -> Self {
        Self {
            query,
            filters: vec![],
            ..Default::default()
        }
    }

    // from_method
    #[must_use]
    pub fn from_method(method: LoadMethod) -> Self {
        let query = LoadQuery::new(method);

        Self {
            query,
            filters: vec![],
            ..Default::default()
        }
    }

    // search
    #[must_use]
    pub fn search(mut self, search: &[(String, String)]) -> Self {
        self.query.search = search.to_vec();
        self
    }

    // order
    pub fn order<T: Into<Order>>(mut self, order: T) -> Self {
        self.query.order = Some(order.into());
        self
    }

    // order_option
    pub fn order_option<T: Into<Order>>(mut self, order: Option<T>) -> Self {
        self.query.order = order.map(Into::into);
        self
    }

    // filter
    pub fn filter<F: Fn(&E) -> bool + 'static>(mut self, f: F) -> Self {
        self.filters.push(Box::new(f));
        self
    }

    // filter_eq
    pub fn filter_eq<F, T>(self, f: F, expected: T) -> Self
    where
        F: Fn(&E) -> T + 'static,
        T: PartialEq + 'static,
    {
        self.filter(move |e| f(e) == expected)
    }

    // filter_some_eq
    pub fn filter_some_eq<F, T>(self, f: F, value: T) -> Self
    where
        F: Fn(&E) -> Option<T> + 'static,
        T: PartialEq + 'static,
    {
        self.filter(move |e| f(e).as_ref() == Some(&value))
    }

    // execute
    // excutes the query and returns a collection
    pub fn execute(self, db: DbLocal) -> Result<LoadCollection<E>, Error> {
        let executor = LoadQueryExecutor::<E>::new(self);
        executor.execute(db)
    }

    // response
    pub fn response(self, db: DbLocal) -> Result<LoadResponse, Error> {
        let executor = LoadQueryExecutor::<E>::new(self);
        executor.response(db)
    }
}

impl<E> LoadQueryBuilderTrait for LoadQueryBuilder<E>
where
    E: Entity,
{
    fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    fn format(mut self, format: LoadFormat) -> Self {
        self.query.format = format;
        self
    }

    fn offset(mut self, offset: u32) -> Self {
        self.query.offset = offset;
        self
    }

    fn limit(mut self, limit: u32) -> Self {
        self.query.limit = Some(limit);
        self
    }

    fn limit_option(mut self, limit: Option<u32>) -> Self {
        self.query.limit = limit;
        self
    }
}

///
/// LoadQueryExecutor
///

pub struct LoadQueryExecutor<E>
where
    E: Entity,
{
    builder: LoadQueryBuilder<E>,
}

impl<E> LoadQueryExecutor<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub const fn new(builder: LoadQueryBuilder<E>) -> Self {
        Self { builder }
    }

    // execute
    pub fn execute(self, db: DbLocal) -> Result<LoadCollection<E>, Error> {
        let query = &self.builder.query;

        self.builder
            .debug
            .println(&format!("query.load: {query:?}"));

        // loader
        let resolver = Resolver::new(E::PATH);
        let loader = Loader::new(db, resolver);
        let res = loader.load(&query.method)?;

        // convert
        let rows = res
            .into_iter()
            .filter(|row| row.value.path == E::path())
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()
            .map_err(QueryError::SerializeError)?;

        // search and filters
        let rows = rows
            .into_iter()
            .filter(|row| {
                let entity = &row.value.entity;

                // run query.search
                let matches_search = if query.search.is_empty() {
                    true
                } else {
                    entity.search_fields(&query.search)
                };

                // run additional filters
                let matches_all_closures = self
                    .builder
                    .filters
                    .iter()
                    .all(|filter_fn| filter_fn(entity));

                matches_search && matches_all_closures
            })
            .collect::<Vec<_>>();

        // sort
        let mut rows = rows;
        if let Some(order) = &query.order {
            let sorter = E::sort(order);
            rows.sort_by(|a, b| sorter(&a.value.entity, &b.value.entity));
        }

        // offset and limit
        let rows = rows
            .into_iter()
            .skip(query.offset as usize)
            .take(query.limit.unwrap_or(u32::MAX) as usize)
            .collect();

        Ok(LoadCollection(rows))
    }

    // response
    pub fn response(self, db: DbLocal) -> Result<LoadResponse, Error> {
        let format = self.builder.query.format.clone();
        let collection = self.execute(db)?;

        let response = match format {
            LoadFormat::Rows => LoadResponse::Rows(collection.data_rows()),
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
    // as_dyn
    #[must_use]
    pub fn as_dyn(self) -> LoadCollectionDyn {
        let data_rows: Vec<DataRow> = self
            .0
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect();

        LoadCollectionDyn(data_rows)
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

impl<E> LoadCollectionTrait for LoadCollection<E>
where
    E: Entity,
{
    fn count(self) -> usize {
        self.0.len()
    }

    fn key(self) -> Option<SortKey> {
        self.0.first().map(|row| row.key.clone())
    }

    fn try_key(self) -> Result<SortKey, QueryError> {
        let row = self.0.first().ok_or(LoadError::NoResultsFound)?;

        Ok(row.key.clone())
    }

    fn keys(self) -> Vec<SortKey> {
        self.0.into_iter().map(|row| row.key).collect()
    }

    fn data_row(self) -> Option<DataRow> {
        self.as_dyn().data_row()
    }

    fn try_data_row(self) -> Result<DataRow, QueryError> {
        self.as_dyn().try_data_row()
    }

    fn data_rows(self) -> Vec<DataRow> {
        self.as_dyn().data_rows()
    }

    fn blob(self) -> Option<Vec<u8>> {
        self.as_dyn().blob()
    }

    fn try_blob(self) -> Result<Vec<u8>, QueryError> {
        self.as_dyn().try_blob()
    }

    fn blobs(self) -> Vec<Vec<u8>> {
        self.as_dyn().blobs()
    }
}
