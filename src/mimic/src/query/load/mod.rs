pub mod dynamic;
pub mod generic;

pub use dynamic::{LoadBuilderDyn, LoadExecutorDyn, LoadQueryDyn};
pub use generic::{LoadBuilder, LoadExecutor, LoadQuery};

use crate::{
    Error, ThisError,
    db::{
        DbError, DbLocal, StoreLocal,
        types::{DataKey, DataRow, EntityRow},
    },
    ic::serialize::SerializeError,
    orm::traits::Entity,
    query::{
        QueryError,
        resolver::{Resolver, ResolverError},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///
/// LoadMap
///

pub type LoadMap<E> = HashMap<String, E>;

///
/// LoadError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum LoadError {
    #[error("key not found: {0}")]
    KeyNotFound(DataKey),

    #[error("no results found")]
    NoResultsFound,

    #[error("response has no entity data")]
    ResponseHasNoEntityData,

    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    ResolverError(#[from] ResolverError),
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

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum LoadMethod {
    All,
    Only,
    One(Vec<String>),
    Many(Vec<Vec<String>>),
    Prefix(Vec<String>),
    Range(Vec<String>, Vec<String>),
}

///
/// LoadFormat
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub enum LoadFormat {
    #[default]
    DataRows,
    Keys,
    Count,
}

///
/// LoadResponse
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub enum LoadResponse {
    DataRows(Vec<DataRow>),
    Keys(Vec<DataKey>),
    Count(usize),
}

impl LoadResponse {
    pub fn as_entity_rows<E: Entity>(&self) -> Result<Vec<EntityRow<E>>, Error> {
        let convert_err = |e| Error::QueryError(QueryError::LoadError(e));

        match self {
            LoadResponse::DataRows(rows) => rows
                .clone()
                .into_iter()
                .map(|row| {
                    row.try_into()
                        .map_err(LoadError::SerializeError)
                        .map_err(convert_err)
                })
                .collect(),

            _ => Err(convert_err(LoadError::ResponseHasNoEntityData)),
        }
    }
}

///
/// Loader
/// took logic from both Load types and stuck it here
///

pub struct Loader {
    db: DbLocal,
    resolver: Resolver,
}

impl Loader {
    // new
    #[must_use]
    pub const fn new(db: DbLocal, resolver: Resolver) -> Self {
        Self { db, resolver }
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
        let store_path = &self.resolver.store()?;
        let store = self.db.with(|db| db.try_get_store(store_path))?;

        let res = match method {
            LoadMethod::All | LoadMethod::Only => {
                let start = self.data_key(&[])?;
                let end = start.create_upper_bound();

                query_range(store, start, end)
            }

            LoadMethod::One(ck) => {
                let key = self.data_key(ck)?;
                let row = query_data_key(store, key)?;

                vec![row]
            }

            LoadMethod::Many(cks) => {
                let keys = cks
                    .iter()
                    .map(|ck| self.data_key(ck))
                    .collect::<Result<Vec<_>, _>>()?;

                keys.into_iter()
                    .map(|key| query_data_key(store, key))
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>()
            }

            LoadMethod::Prefix(prefix) => {
                let start = self.data_key(prefix)?;
                let end = start.create_upper_bound();

                query_range(store, start, end)
            }

            LoadMethod::Range(start_ck, end_ck) => {
                let start = self.data_key(start_ck)?;
                let end = self.data_key(end_ck)?;

                query_range(store, start, end)
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
}

// query_data_key
fn query_data_key(store: StoreLocal, key: DataKey) -> Result<DataRow, LoadError> {
    store.with_borrow(|this| {
        this.get(&key)
            .map(|value| DataRow {
                key: key.clone(),
                value,
            })
            .ok_or_else(|| LoadError::KeyNotFound(key.clone()))
    })
}

// query_range
fn query_range(store: StoreLocal, start: DataKey, end: DataKey) -> Vec<DataRow> {
    store.with_borrow(|this| {
        this.range(start..=end)
            .map(|(key, value)| DataRow { key, value })
            .collect()
    })
}
