use crate::{
    Error,
    db::{DbLocal, StoreLocal, types::DataKey},
    query::{DebugContext, QueryError, Resolver},
};
use crate::{ThisError, db::DbError, query::resolver::ResolverError};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

///
/// DeleteError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum DeleteError {
    #[error("key not found: {0}")]
    KeyNotFound(DataKey),

    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    ResolverError(#[from] ResolverError),
}

///
/// DeleteMethod
///
/// One  : one key
/// Many : many keys
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum DeleteMethod {
    One(Vec<String>),
    Many(Vec<Vec<String>>),
}

///
/// DeleteBuilder
///

#[derive(Default)]
pub struct DeleteBuilder {
    path: String,
}

impl DeleteBuilder {
    // new
    #[must_use]
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    // one
    pub fn one<T: Display>(self, ck: &[T]) -> DeleteQuery {
        let key = ck.iter().map(ToString::to_string).collect();

        DeleteQuery::new(&self.path, DeleteMethod::One(key))
    }

    // many
    #[must_use]
    pub fn many<T: Display>(self, ck: &[Vec<T>]) -> DeleteQuery {
        let keys: Vec<Vec<String>> = ck
            .iter()
            .map(|inner_vec| inner_vec.iter().map(ToString::to_string).collect())
            .collect();

        DeleteQuery::new(&self.path, DeleteMethod::Many(keys))
    }
}

///
/// DeleteQuery
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct DeleteQuery {
    path: String,
    method: DeleteMethod,
    debug: DebugContext,
}

impl DeleteQuery {
    // new
    #[must_use]
    pub fn new(path: &str, method: DeleteMethod) -> Self {
        Self {
            path: path.to_string(),
            method,
            debug: DebugContext::default(),
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // execute
    pub fn execute(self, db: DbLocal) -> Result<DeleteResponse, Error> {
        let executor = DeleteExecutor::new(self);

        executor.execute(db)
    }
}

///
/// DeleteExecutor
///

pub struct DeleteExecutor {
    query: DeleteQuery,
    resolver: Resolver,
}

impl DeleteExecutor {
    // new
    #[must_use]
    pub fn new(query: DeleteQuery) -> Self {
        let resolver = Resolver::new(&query.path);

        Self { query, resolver }
    }

    // execute
    pub fn execute(&self, db: DbLocal) -> Result<DeleteResponse, Error> {
        let mut keys_deleted = Vec::new();

        let keys = match self.query.method.clone() {
            DeleteMethod::One(key) => vec![key],
            DeleteMethod::Many(keys) => keys,
        };
        crate::ic::println!("delete: keys {:?}", &keys);

        // get store
        let store_path = &self
            .resolver
            .store()
            .map_err(DeleteError::ResolverError)
            .map_err(QueryError::DeleteError)?;

        let store = db.with(|db| db.try_get_store(store_path))?;

        // execute for every different key
        for key in keys {
            let res = self.execute_one(store, key)?;

            keys_deleted.push(res);
        }

        // debug
        self.query
            .debug
            .println(&format!("deleted keys {keys_deleted:?}"));

        Ok(DeleteResponse(keys_deleted))
    }

    // execute_one
    fn execute_one(&self, store: StoreLocal, key: Vec<String>) -> Result<DataKey, Error> {
        // Attempt to remove the item from the store
        let data_key = self
            .resolver
            .data_key(&key)
            .map_err(DeleteError::ResolverError)
            .map_err(QueryError::DeleteError)?;

        store.with_borrow_mut(|store| {
            if store.remove(&data_key).is_none() {
                crate::ic::println!("key {data_key:?} not found");
            }

            Ok::<_, Error>(())
        })?;

        Ok(data_key)
    }
}

///
/// DeleteResponse
///

#[derive(CandidType, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct DeleteResponse(Vec<DataKey>);
