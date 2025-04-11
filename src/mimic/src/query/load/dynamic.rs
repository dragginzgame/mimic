use crate::{
    Error,
    db::{DbLocal, types::DataRow},
    orm::{base::types::SortKey, traits::Entity},
    query::{
        DebugContext, QueryError, Resolver,
        load::{LoadError, LoadFormat, LoadMethod, LoadResponse, Loader},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

///
/// LoadBuilderDyn
///

#[derive(Debug, Default)]
pub struct LoadBuilderDyn<E>
where
    E: Entity,
{
    phantom: PhantomData<E>,
}

impl<E> LoadBuilderDyn<E>
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
    pub fn method(self, method: LoadMethod) -> LoadQueryDyn {
        LoadQueryDyn::new(E::PATH, method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryDyn {
        LoadQueryDyn::new(E::PATH, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryDyn {
        LoadQueryDyn::new(E::PATH, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQueryDyn {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQueryDyn::new(E::PATH, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQueryDyn {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQueryDyn::new(E::PATH, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQueryDyn {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        LoadQueryDyn::new(E::PATH, method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQueryDyn {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        LoadQueryDyn::new(E::PATH, method)
    }
}

///
/// LoadQueryDyn
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct LoadQueryDyn {
    path: String,
    method: LoadMethod,
    format: LoadFormat,
    offset: u32,
    limit: Option<u32>,
    debug: DebugContext,
}

impl LoadQueryDyn {
    // new
    #[must_use]
    pub fn new(path: &str, method: LoadMethod) -> Self {
        Self {
            path: path.to_string(),
            method,
            format: LoadFormat::default(),
            offset: 0,
            limit: None,
            debug: DebugContext::default(),
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // format
    #[must_use]
    pub const fn format(mut self, format: LoadFormat) -> Self {
        self.format = format;
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

    // execute
    pub fn execute(self, db: DbLocal) -> Result<LoadCollectionDyn, Error> {
        let executor = LoadExecutorDyn::new(self);
        executor.execute(db)
    }

    // response
    pub fn response(self, db: DbLocal) -> Result<LoadResponse, Error> {
        let executor = LoadExecutorDyn::new(self);
        executor.response(db)
    }
}

///
/// LoadExecutorDyn
///

pub struct LoadExecutorDyn {
    query: LoadQueryDyn,
    resolver: Resolver,
}

impl LoadExecutorDyn {
    // new
    #[must_use]
    pub fn new(query: LoadQueryDyn) -> Self {
        let resolver = Resolver::new(&query.path);

        Self { query, resolver }
    }

    // execute
    pub fn execute(self, db: DbLocal) -> Result<LoadCollectionDyn, Error> {
        // loader
        let loader = Loader::new(db, self.resolver);
        let rows = loader.load(&self.query.method)?;

        let filtered_rows = rows
            .into_iter()
            .skip(self.query.offset as usize)
            .take(self.query.limit.unwrap_or(u32::MAX) as usize)
            .collect::<Vec<_>>();

        Ok(LoadCollectionDyn(filtered_rows))
    }

    // response
    pub fn response(self, db: DbLocal) -> Result<LoadResponse, Error> {
        let format = self.query.format.clone();
        let collection = self.execute(db)?;

        let response = match format {
            LoadFormat::DataRows => LoadResponse::DataRows(collection.data_rows()),
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

impl LoadCollectionDyn {
    // count
    #[must_use]
    pub fn count(self) -> usize {
        self.0.len()
    }

    // data_row
    #[must_use]
    pub fn data_row(self) -> Option<DataRow> {
        self.0.first().cloned()
    }

    // data_rows
    #[must_use]
    pub fn data_rows(self) -> Vec<DataRow> {
        self.0
    }

    // key
    #[must_use]
    pub fn key(self) -> Option<SortKey> {
        self.0.first().map(|row| row.key.clone())
    }

    // try_key
    pub fn try_key(self) -> Result<SortKey, Error> {
        let row = self
            .0
            .first()
            .ok_or(LoadError::NoResultsFound)
            .map_err(QueryError::LoadError)?;

        Ok(row.key.clone())
    }

    // keys
    #[must_use]
    pub fn keys(self) -> Vec<SortKey> {
        self.0.into_iter().map(|row| row.key).collect()
    }

    // try_blob
    pub fn try_blob(self) -> Result<Vec<u8>, Error> {
        self.0
            .into_iter()
            .next()
            .map(|row| row.value.data)
            .ok_or_else(|| QueryError::LoadError(LoadError::NoResultsFound).into())
    }

    // blob
    #[must_use]
    pub fn blob(self) -> Option<Vec<u8>> {
        self.0.first().map(|row| row.value.data.clone())
    }

    // blobs
    #[must_use]
    pub fn blobs(self) -> Vec<Vec<u8>> {
        self.0.into_iter().map(|row| row.value.data).collect()
    }
}
