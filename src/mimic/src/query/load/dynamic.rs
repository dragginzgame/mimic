use crate::{
    Error,
    db::{
        DbLocal,
        types::{DataRow, SortKey},
    },
    query::{
        DebugContext, QueryError, Resolver,
        load::{LoadError, LoadFormat, LoadMethod, LoadQuery, LoadResponse, Loader},
        traits::{LoadCollectionTrait, LoadQueryBuilderTrait},
        types::{Order, Search},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadQueryDynInit
///

#[derive(Debug, Default)]
pub struct LoadQueryDynInit {}

impl LoadQueryDynInit {
    // new
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    // query
    #[must_use]
    pub fn query(self, query: LoadQuery) -> LoadQueryDynBuilder {
        LoadQueryDynBuilder::new(query)
    }

    // method
    #[must_use]
    pub fn method(self, path: &str, method: LoadMethod) -> LoadQueryDynBuilder {
        LoadQueryDynBuilder::new_with(path, method)
    }

    // all
    #[must_use]
    pub fn all(self, path: &str) -> LoadQueryDynBuilder {
        LoadQueryDynBuilder::new_with(path, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self, path: &str) -> LoadQueryDynBuilder {
        LoadQueryDynBuilder::new_with(path, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, path: &str, ck: &[T]) -> LoadQueryDynBuilder {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQueryDynBuilder::new_with(path, method)
    }

    // many
    #[must_use]
    pub fn many(self, path: &str, cks: &[Vec<String>]) -> LoadQueryDynBuilder {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQueryDynBuilder::new_with(path, method)
    }

    // range
    pub fn range<T: ToString>(self, path: &str, start: &[T], end: &[T]) -> LoadQueryDynBuilder {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        LoadQueryDynBuilder::new_with(path, method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, path: &str, prefix: &[T]) -> LoadQueryDynBuilder {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        LoadQueryDynBuilder::new_with(path, method)
    }
}

///
/// LoadQueryDynBuilder
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct LoadQueryDynBuilder {
    query: LoadQuery,
    debug: DebugContext,
}

impl LoadQueryDynBuilder {
    // new
    #[must_use]
    pub fn new(query: LoadQuery) -> Self {
        Self {
            query,
            debug: DebugContext::default(),
        }
    }

    // new_with
    #[must_use]
    pub fn new_with(path: &str, method: LoadMethod) -> Self {
        let query = LoadQuery::new(path, method);

        Self {
            query,
            debug: DebugContext::default(),
        }
    }

    // execute
    pub fn execute(self, db: DbLocal) -> Result<LoadCollectionDyn, Error> {
        let executor = LoadQueryDynExecutor::new(self);
        executor.execute(db)
    }

    // response
    pub fn response(self, db: DbLocal) -> Result<LoadResponse, Error> {
        let executor = LoadQueryDynExecutor::new(self);
        executor.response(db)
    }
}

impl LoadQueryBuilderTrait for LoadQueryDynBuilder {
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

    fn search<S: Into<Search>>(mut self, search: S) -> Self {
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
/// LoadQueryDynExecutor
///

pub struct LoadQueryDynExecutor {
    builder: LoadQueryDynBuilder,
    resolver: Resolver,
}

impl LoadQueryDynExecutor {
    // new
    #[must_use]
    pub fn new(builder: LoadQueryDynBuilder) -> Self {
        let resolver = Resolver::new(&builder.query.path);

        Self { builder, resolver }
    }

    // execute
    pub fn execute(self, db: DbLocal) -> Result<LoadCollectionDyn, Error> {
        let query = &self.builder.query;

        // loader
        let loader = Loader::new(db, self.resolver);
        let rows = loader.load(&query.method)?;

        let filtered_rows = rows
            .into_iter()
            .skip(query.offset as usize)
            .take(query.limit.unwrap_or(u32::MAX) as usize)
            .collect::<Vec<_>>();

        Ok(LoadCollectionDyn(filtered_rows))
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
/// LoadCollectionDyn
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct LoadCollectionDyn(pub Vec<DataRow>);

impl LoadCollectionTrait for LoadCollectionDyn {
    fn count(self) -> usize {
        self.0.len()
    }

    fn key(self) -> Option<SortKey> {
        self.0.first().map(|row| row.key.clone())
    }

    fn try_key(self) -> Result<SortKey, QueryError> {
        let row = self
            .0
            .first()
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(row.key.clone())
    }

    fn keys(self) -> Vec<SortKey> {
        self.0.into_iter().map(|row| row.key).collect()
    }

    fn data_row(self) -> Option<DataRow> {
        self.0.first().cloned()
    }

    fn try_data_row(self) -> Result<DataRow, QueryError> {
        let row = self
            .0
            .first()
            .cloned()
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(row)
    }

    fn data_rows(self) -> Vec<DataRow> {
        self.0
    }

    fn blob(self) -> Option<Vec<u8>> {
        self.0.first().map(|row| row.value.data.clone())
    }

    fn try_blob(self) -> Result<Vec<u8>, QueryError> {
        self.0
            .into_iter()
            .next()
            .map(|row| row.value.data)
            .ok_or(QueryError::LoadError(LoadError::NoResultsFound))
    }

    fn blobs(self) -> Vec<Vec<u8>> {
        self.0.into_iter().map(|row| row.value.data).collect()
    }
}
