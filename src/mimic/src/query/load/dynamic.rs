use crate::{
    Error,
    db::{
        DbLocal,
        types::{DataRow, SortKey},
    },
    query::{
        DebugContext, QueryError, Resolver,
        load::{LoadError, LoadFormat, LoadMethod, LoadResponse, Loader},
        traits::{LoadCollectionTrait, LoadQueryBuilderTrait},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadQueryDyn
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoadQueryDyn {
    pub path: String,
    pub method: LoadMethod,
    pub format: LoadFormat,
    pub offset: u32,
    pub limit: Option<u32>,
}

impl LoadQueryDyn {
    #[must_use]
    pub fn new(path: &str, method: LoadMethod) -> Self {
        Self {
            path: path.to_string(),
            method,
            ..Default::default()
        }
    }
}

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
    pub fn query(self, query: LoadQueryDyn) -> LoadQueryDynBuilder {
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

pub struct LoadQueryDynBuilder {
    query: LoadQueryDyn,
    debug: DebugContext,
}

impl LoadQueryDynBuilder {
    // new
    #[must_use]
    pub fn new(query: LoadQueryDyn) -> Self {
        Self {
            query,
            debug: DebugContext::default(),
        }
    }

    // new_with
    #[must_use]
    pub fn new_with(path: &str, method: LoadMethod) -> Self {
        Self {
            query: LoadQueryDyn::new(path, method),
            debug: DebugContext::default(),
        }
    }

    // query
    #[must_use]
    pub fn query(self) -> LoadQueryDyn {
        self.query
    }

    // execute
    pub fn execute(self, db: DbLocal) -> Result<LoadCollectionDyn, Error> {
        let executor = LoadQueryDynExecutor::new(self.query, self.debug);
        executor.execute(db)
    }

    // response
    pub fn response(self, db: DbLocal) -> Result<LoadResponse, Error> {
        let executor = LoadQueryDynExecutor::new(self.query, self.debug);
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
}

///
/// LoadQueryDynExecutor
///

pub struct LoadQueryDynExecutor {
    query: LoadQueryDyn,
    debug: DebugContext,
    resolver: Resolver,
}

impl LoadQueryDynExecutor {
    // new
    #[must_use]
    pub fn new(query: LoadQueryDyn, debug: DebugContext) -> Self {
        let resolver = Resolver::new(&query.path);

        Self {
            query,
            debug,
            resolver,
        }
    }

    // execute
    pub fn execute(self, db: DbLocal) -> Result<LoadCollectionDyn, Error> {
        let query = &self.query;

        self.debug.println(&format!("query.load_dyn: {query:?}"));

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
        let format = self.query.format.clone();
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
