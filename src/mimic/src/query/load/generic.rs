use crate::{
    Error,
    db::{
        DbLocal,
        types::{DataRow, EntityRow, EntityValue, SortKey},
    },
    query::{
        DebugContext, QueryError, Resolver,
        load::{
            LoadCollectionDyn, LoadError, LoadFormat, LoadMap, LoadMethod, LoadQuery, LoadResponse,
            Loader,
        },
        traits::{LoadCollectionTrait, LoadQueryBuilderTrait},
        types::{Order, Search},
    },
    traits::Entity,
};
use std::marker::PhantomData;

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
        LoadQueryBuilder::new(method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryBuilder<E> {
        LoadQueryBuilder::new(LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryBuilder<E> {
        LoadQueryBuilder::new(LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQueryBuilder<E> {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQueryBuilder::new(method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQueryBuilder<E> {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQueryBuilder::new(method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQueryBuilder<E> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        LoadQueryBuilder::new(method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQueryBuilder<E> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        LoadQueryBuilder::new(method)
    }
}

///
/// LoadQueryBuilder
/// we don't allow passing a LoadQuery because we already know the path
///

#[allow(clippy::type_complexity)]
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
    pub fn new(method: LoadMethod) -> Self {
        let query = LoadQuery::new(E::PATH, method);

        Self {
            query,
            filters: vec![],
            debug: DebugContext::default(),
        }
    }

    // filter
    pub fn filter<F: Fn(&E) -> bool + 'static>(mut self, f: F) -> Self {
        self.filters.push(Box::new(f));
        self
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

    fn search<T: Into<Search>>(mut self, search: T) -> Self {
        self.query.search = Some(search.into());
        self
    }

    fn search_option(mut self, search: Option<Search>) -> Self {
        self.query.search = search;
        self
    }

    fn search_all(mut self, text: &str) -> Self {
        self.query.search = Some(Search::all(text.to_string()));
        self
    }

    fn search_fields<I, K, V>(mut self, fields: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.query.search = Some(Search::fields(fields));
        self
    }

    fn order<T: Into<Order>>(mut self, order: T) -> Self {
        self.query.order = Some(order.into());
        self
    }

    fn order_option<T: Into<Order>>(mut self, order: Option<T>) -> Self {
        self.query.order = order.map(Into::into);
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

        // loader
        let resolver = Resolver::new(&query.path);
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
                let matches_search = match &query.search {
                    Some(Search::All(text)) => entity.search_all(text),
                    Some(Search::Fields(fields)) => entity.search_fields(fields.clone()),
                    None => true,
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
