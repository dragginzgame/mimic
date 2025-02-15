pub mod dynamic;
pub mod generic;
pub mod result;

pub use dynamic::{LoadBuilderDyn, LoadExecutorDyn, LoadQueryDyn};
pub use generic::{LoadBuilder, LoadExecutor, LoadQuery};
pub use result::{LoadResult, LoadResultDyn};

use crate::{
    Error, ThisError,
    db::{
        DbError, StoreLocal,
        types::{DataKey, DataRow},
    },
    ic::serialize::SerializeError,
    query::{
        QueryError,
        resolver::{Resolver, ResolverError},
        types::{Filter, Order},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum LoadError {
    #[error("method can only be set once")]
    MethodAlreadySet,

    #[error("method is not set")]
    MethodNotSet,

    #[error("key not found: {0}")]
    KeyNotFound(DataKey),

    #[error("no results found")]
    NoResultsFound,

    #[error("range queries not allowed on composite keys")]
    RangeNotAllowed,

    #[error(transparent)]
    DbError(#[from] DbError),

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

pub struct Loader {
    store: StoreLocal,
    resolver: Resolver,
}

impl Loader {
    // new
    #[must_use]
    pub const fn new(store: StoreLocal, resolver: Resolver) -> Self {
        Self { store, resolver }
    }

    // load
    pub fn load(&self, method: &LoadMethod) -> Result<Vec<DataRow>, Error> {
        self.load_unmapped(method)
            .map_err(QueryError::LoadError)
            .map_err(Error::QueryError)
    }

    // load_unmapped
    // for easier error wrapping
    fn load_unmapped(&self, method: &LoadMethod) -> Result<Vec<DataRow>, LoadError> {
        let res = match method {
            LoadMethod::All | LoadMethod::Only => {
                let start = self.data_key(&[])?;
                let end = start.create_upper_bound();

                self.query_range(start, end)
            }

            LoadMethod::One(ck) => {
                let key = self.data_key(ck)?;
                let row = self.query_data_key(key)?;

                vec![row]
            }

            LoadMethod::Many(cks) => {
                let keys = cks
                    .iter()
                    .map(|ck| self.data_key(ck))
                    .collect::<Result<Vec<_>, _>>()?;

                keys.into_iter()
                    .map(|key| self.query_data_key(key))
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>()
            }

            LoadMethod::Prefix(prefix) => {
                let start = self.data_key(prefix)?;
                let end = start.create_upper_bound();

                self.query_range(start, end)
            }

            LoadMethod::Range(start_ck, end_ck) => {
                let start = self.data_key(start_ck)?;
                let end = self.data_key(end_ck)?;

                self.query_range(start, end)
            }
        };

        Ok(res)
    }

    // data_key
    // for easy error converstion
    fn data_key(&self, ck: &[String]) -> Result<DataKey, LoadError> {
        let key = self.resolver.data_key(ck)?;

        Ok(key)
    }

    // query_data_key
    fn query_data_key(&self, key: DataKey) -> Result<DataRow, LoadError> {
        self.store.with_borrow(|this| {
            this.get(&key)
                .map(|value| DataRow {
                    key: key.clone(),
                    value,
                })
                .ok_or_else(|| LoadError::KeyNotFound(key.clone()))
        })
    }

    // query_range
    fn query_range(&self, start: DataKey, end: DataKey) -> Vec<DataRow> {
        self.store.with_borrow(|this| {
            this.range(start..=end)
                .map(|(key, value)| DataRow { key, value })
                .collect()
        })
    }
}
