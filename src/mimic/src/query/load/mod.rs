pub mod dynamic;
pub mod result;
pub mod r#static;

pub use dynamic::{LoadBuilderDyn, LoadExecutorDyn, LoadQueryDyn};
pub use r#static::{LoadBuilder, LoadExecutor, LoadQuery};
pub use result::{LoadResult, LoadResultDyn};

use crate::{
    orm::serialize::SerializeError,
    query::{
        resolver::{Resolver, ResolverError},
        types::LoadMethod,
        QueryError,
    },
    store::{
        types::{DataKey, DataRow},
        StoreLocal,
    },
    Error, ThisError,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum LoadError {
    #[error("key not found: {0}")]
    KeyNotFound(DataKey),

    #[error("no results found")]
    NoResultsFound,

    #[error("range queries not allowed on composite keys")]
    RangeNotAllowed,

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    ResolverError(#[from] ResolverError),
}

///
/// Loader
/// took logic from both Load types and stuck it here
///

pub struct Loader<'a> {
    store: StoreLocal,
    resolver: &'a Resolver,
}

impl<'a> Loader<'a> {
    // new
    #[must_use]
    pub fn new(store: StoreLocal, resolver: &'a Resolver) -> Self {
        Self { store, resolver }
    }

    // load
    pub fn load(&self, method: &LoadMethod) -> Result<Box<dyn Iterator<Item = DataRow>>, Error> {
        match method {
            LoadMethod::All | LoadMethod::Only => {
                let start = self.data_key(&[])?;
                let end = start.create_upper_bound();
                let rows = self.query_range(start, end);

                Ok(Box::new(rows.into_iter()))
            }

            LoadMethod::One(ck) => {
                let key = self.data_key(ck)?;
                let res = self.query_data_key(key)?;

                Ok(Box::new(std::iter::once(res)))
            }

            LoadMethod::Many(cks) => {
                let keys = cks
                    .iter()
                    .map(|ck| self.data_key(ck))
                    .collect::<Result<Vec<_>, _>>()?;

                let rows = keys
                    .into_iter()
                    .map(|key| self.query_data_key(key))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Box::new(rows.into_iter()))
            }

            LoadMethod::Prefix(prefix) => {
                let start = self.data_key(prefix)?;
                let end = start.create_upper_bound();
                let rows = self.query_range(start, end);

                Ok(Box::new(rows.into_iter()))
            }

            LoadMethod::Range(start_ck, end_ck) => {
                let start = self.data_key(start_ck)?;
                let end = self.data_key(end_ck)?;
                let rows = self.query_range(start, end);

                Ok(Box::new(rows.into_iter()))
            }
        }
    }

    // data_key
    // for easy error converstion
    fn data_key(&self, ck: &[String]) -> Result<DataKey, Error> {
        let key = self
            .resolver
            .data_key(ck)
            .map_err(LoadError::ResolverError)
            .map_err(QueryError::LoadError)?;

        Ok(key)
    }

    // query_data_key
    fn query_data_key(&self, key: DataKey) -> Result<DataRow, Error> {
        self.store.with_borrow(|store| {
            let row = store
                .data
                .get(&key)
                .map(|value| DataRow {
                    key: key.clone(),
                    value,
                })
                .ok_or(LoadError::KeyNotFound(key))
                .map_err(QueryError::LoadError)?;

            Ok(row)
        })
    }

    // query_range
    fn query_range(&self, start: DataKey, end: DataKey) -> Vec<DataRow> {
        self.store.with_borrow(|store| {
            store
                .data
                .range(start..=end)
                .map(|(key, value)| DataRow { key, value })
                .collect()
        })
    }
}
