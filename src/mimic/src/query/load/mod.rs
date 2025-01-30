pub mod dynamic;
pub mod result;
pub mod r#static;

pub use dynamic::{LoadBuilder, LoadExecutor, LoadQuery};
pub use r#static::{ELoadBuilder, ELoadExecutor, ELoadQuery};
pub use result::{ELoadResult, LoadResult};

use crate::{
    orm::OrmError,
    query::{
        resolver::{Resolver, ResolverError},
        types::LoadMethod,
    },
    store::{
        types::{DataKey, DataRow},
        StoreLocal,
    },
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// LoadError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum LoadError {
    #[snafu(display("key not found: {key}"))]
    KeyNotFound { key: DataKey },

    #[snafu(display("no results found"))]
    NoResultsFound,

    #[snafu(display("range queries not allowed on composite keys"))]
    RangeNotAllowed,

    #[snafu(transparent)]
    OrmError { source: OrmError },

    #[snafu(transparent)]
    ResolverError { source: ResolverError },
}

///
/// Loader
/// took logic from both Load types and stuck it here
///

pub struct Loader {}

impl Loader {
    // load
    pub fn load(
        store: StoreLocal,
        resolver: &Resolver,
        method: &LoadMethod,
    ) -> Result<Box<dyn Iterator<Item = DataRow>>, LoadError> {
        match method {
            LoadMethod::All | LoadMethod::Only => {
                let start = resolver.data_key(&[])?;
                let end = start.create_upper_bound();
                let rows = Self::query_range(store, start, end);

                Ok(Box::new(rows.into_iter()))
            }

            LoadMethod::One(ck) => {
                let key = resolver.data_key(ck)?;
                let res = Self::query_data_key(store, key)?;

                Ok(Box::new(std::iter::once(res)))
            }

            LoadMethod::Many(cks) => {
                let keys = cks
                    .iter()
                    .map(|ck| resolver.data_key(ck))
                    .collect::<Result<Vec<_>, _>>()?;

                let rows = keys
                    .into_iter()
                    .map(|key| Self::query_data_key(store, key))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Box::new(rows.into_iter()))
            }

            LoadMethod::Prefix(prefix) => {
                let start = resolver.data_key(prefix)?;
                let end = start.create_upper_bound();
                let rows = Self::query_range(store, start, end);

                Ok(Box::new(rows.into_iter()))
            }

            LoadMethod::Range(start_ck, end_ck) => {
                let start = resolver.data_key(start_ck)?;
                let end = resolver.data_key(end_ck)?;
                let rows = Self::query_range(store, start, end);

                Ok(Box::new(rows.into_iter()))
            }
        }
    }

    // query_data_key
    fn query_data_key(store: StoreLocal, key: DataKey) -> Result<DataRow, LoadError> {
        store.with_borrow(|store| {
            store
                .data
                .get(&key)
                .map(|value| DataRow {
                    key: key.clone(),
                    value,
                })
                .ok_or(LoadError::KeyNotFound { key })
        })
    }

    // query_range
    fn query_range(store: StoreLocal, start: DataKey, end: DataKey) -> Vec<DataRow> {
        store.with_borrow(|store| {
            store
                .data
                .range(start..=end)
                .map(|(key, value)| DataRow { key, value })
                .collect()
        })
    }
}
