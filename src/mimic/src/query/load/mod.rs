pub mod generic;
pub mod path;
pub mod result;

pub use generic::{LoadBuilder, LoadExecutor, LoadQuery};
pub use path::{LoadBuilderPath, LoadExecutorPath, LoadQueryPath};
pub use result::{LoadResult, LoadResultDyn};

use crate::{
    ic::serialize::SerializeError,
    query::{
        resolver::{Resolver, ResolverError},
        types::{Filter, Order},
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
/// LoadRequest
/// (from the front end, so no generics)
///
/// entity : Entity path
/// format : the format you want the results in (Rows or Count)
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct LoadRequest {
    pub entity: String,
    pub method: LoadMethod,
    pub offset: u32,
    pub limit: Option<u32>,
    pub filter: Option<Filter>,
    pub order: Option<Order>,
    pub format: LoadFormat,
}

///
/// LoadFormat
///
/// a variant that specifies the format the LoadResponse should be in
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum LoadFormat {
    Rows,
    Count,
}

///
/// LoadMethod
///
/// All    : no sort key prefix, only works with top-level Sort Keys,
///          will probably not work if used on nested entities
/// Only   : for entities that have no keys
/// One    : returns one row by composite key
/// Many   : returns many rows (from many composite keys)
/// Prefix : like all but we're asking for the composite key prefix
///          so Pet (Character=1) will return the Pets from Character 1
/// Range  : user-defined range, ie. Item=1000 Item=1500
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub enum LoadMethod {
    #[default]
    All,
    Only,
    One(Vec<String>),
    Many(Vec<Vec<String>>),
    Prefix(Vec<String>),
    Range(Vec<String>, Vec<String>),
}

///
/// LoadResponse
/// The variant that defines what format the results of a request
/// will be returned in
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum LoadResponse {
    Rows(Vec<DataRow>),
    Count(u32),
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
    pub const fn new(store: StoreLocal, resolver: &'a Resolver) -> Self {
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
