mod dynamic;
mod generic;

pub use dynamic::*;
pub use generic::*;

use crate::{
    Error, ThisError,
    base::types::Relation,
    db::{
        DbLocal, StoreLocal,
        types::{DataRow, EntityRow, SortKey},
    },
    query::{
        QueryError,
        resolver::Resolver,
        types::{Order, Search},
    },
    traits::Entity,
};
use candid::CandidType;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap};

///
/// LoadError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum LoadError {
    #[error("key not found: {0}")]
    KeyNotFound(SortKey),

    #[error("relation not found: {0}")]
    RelationNotFound(Relation),

    #[error("no results found")]
    NoResultsFound,

    #[error("response has no entity data")]
    ResponseHasNoEntityData,
}

///
/// LoadQuery
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct LoadQuery {
    pub path: String,
    pub method: LoadMethod,
    pub format: LoadFormat,
    pub offset: u32,
    pub limit: Option<u32>,
    pub search: Option<Search>,
    pub order: Option<Order>,
}

impl LoadQuery {
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
/// LoadFormat
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub enum LoadFormat {
    #[default]
    Rows,
    Keys,
    Count,
}

///
/// LoadResponse
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub enum LoadResponse {
    Rows(Vec<DataRow>),
    Keys(Vec<SortKey>),
    Count(usize),
}

impl LoadResponse {
    pub fn as_entity_rows<E: Entity>(&self) -> Result<Vec<EntityRow<E>>, Error> {
        match self {
            Self::Rows(rows) => rows
                .clone()
                .into_iter()
                .map(|row| {
                    row.try_into()
                        .map_err(|e| Error::QueryError(QueryError::SerializeError(e)))
                })
                .collect(),

            _ => Err(Error::QueryError(QueryError::LoadError(
                LoadError::ResponseHasNoEntityData,
            ))),
        }
    }
}

///
/// LoadMap
/// a HashMap indexed by id to provide an indexed alternative
/// to Vec<Row>
///

#[derive(Debug, Deref)]
pub struct LoadMap<T>(HashMap<Relation, T>);

impl<T> LoadMap<T> {
    // from_pairs
    pub fn from_pairs<I>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (Relation, T)>,
    {
        let map: HashMap<Relation, T> = pairs.into_iter().collect();
        LoadMap(map)
    }

    // get
    pub fn get<R: Borrow<Relation>>(&self, r: R) -> Option<&T> {
        self.0.get(r.borrow())
    }

    // try_get
    pub fn try_get<R: Borrow<Relation>>(&self, r: R) -> Result<&T, Error> {
        let r = r.borrow();

        self.0.get(r).ok_or_else(|| {
            Error::QueryError(QueryError::LoadError(LoadError::RelationNotFound(
                Relation::from(r.clone()),
            )))
        })
    }

    // get_many
    pub fn get_many<Q, I>(&self, keys: I) -> Vec<&T>
    where
        Q: Borrow<Relation>,
        I: IntoIterator<Item = Q>,
    {
        keys.into_iter()
            .filter_map(|k| self.0.get(k.borrow()))
            .collect()
    }

    // try_get_many
    pub fn try_get_many<Q, I>(&self, keys: I) -> Result<Vec<&T>, Error>
    where
        Q: Borrow<Relation>,
        I: IntoIterator<Item = Q>,
    {
        keys.into_iter()
            .map(|k| {
                let rel = k.borrow();
                self.0.get(rel).ok_or_else(|| {
                    Error::QueryError(QueryError::LoadError(LoadError::RelationNotFound(
                        rel.clone(),
                    )))
                })
            })
            .collect()
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
    pub fn load(&self, method: &LoadMethod) -> Result<Vec<DataRow>, QueryError> {
        let res = self.load_unmapped(method)?;

        Ok(res)
    }

    // load_unmapped
    // for easier error wrapping
    fn load_unmapped(&self, method: &LoadMethod) -> Result<Vec<DataRow>, QueryError> {
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
    fn data_key(&self, ck: &[String]) -> Result<SortKey, QueryError> {
        let key = self.resolver.data_key(ck)?;

        Ok(key)
    }
}

// query_data_key
fn query_data_key(store: StoreLocal, key: SortKey) -> Result<DataRow, QueryError> {
    store.with_borrow(|this| {
        this.get(&key)
            .map(|value| DataRow {
                key: key.clone(),
                value,
            })
            .ok_or(QueryError::LoadError(LoadError::KeyNotFound(key)))
    })
}

// query_range
fn query_range(store: StoreLocal, start: SortKey, end: SortKey) -> Vec<DataRow> {
    store.with_borrow(|this| {
        this.range(start..=end)
            .map(|(key, value)| DataRow { key, value })
            .collect()
    })
}
