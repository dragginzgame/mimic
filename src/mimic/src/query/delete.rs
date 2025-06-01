use crate::{
    Error,
    db::{DataStoreRegistryLocal, types::SortKey},
    query::{DebugContext, QueryError, Resolver},
};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// DeleteError
///

#[derive(Debug, ThisError)]
pub enum DeleteError {
    #[error("undefined delete query")]
    Undefined,
}

///
/// DeleteQuery
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct DeleteQuery {
    path: String,
    method: DeleteMethod,
}

impl DeleteQuery {
    // new
    #[must_use]
    pub fn new(path: &str, method: DeleteMethod) -> Self {
        Self {
            path: path.to_string(),
            method,
        }
    }
}

///
/// DeleteMethod
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub enum DeleteMethod {
    #[default]
    Undefined,
    One(Vec<String>),
    Many(Vec<Vec<String>>),
}

///
/// DeleteResponse
///

#[derive(CandidType, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct DeleteResponse(Vec<SortKey>);

///
/// DeleteQueryInit
///

#[derive(Debug, Default)]
pub struct DeleteQueryInit {}

impl DeleteQueryInit {
    // new
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    // query
    #[must_use]
    pub fn query(self, query: DeleteQuery) -> DeleteQueryBuilder {
        DeleteQueryBuilder::new(query)
    }

    // one
    pub fn one<S: ToString>(self, path: &str, ck: &[S]) -> DeleteQueryBuilder {
        let key = ck.iter().map(ToString::to_string).collect();
        let method = DeleteMethod::One(key);

        DeleteQueryBuilder::new_with(path, method)
    }

    // many
    #[must_use]
    pub fn many<S: ToString>(self, path: &str, ck: &[Vec<S>]) -> DeleteQueryBuilder {
        let keys: Vec<Vec<String>> = ck
            .iter()
            .map(|inner_vec| inner_vec.iter().map(ToString::to_string).collect())
            .collect();
        let method = DeleteMethod::Many(keys);

        DeleteQueryBuilder::new_with(path, method)
    }
}

///
/// DeleteQueryBuilder
///

pub struct DeleteQueryBuilder {
    query: DeleteQuery,
    debug: DebugContext,
}

impl DeleteQueryBuilder {
    // new
    #[must_use]
    pub fn new(query: DeleteQuery) -> Self {
        Self {
            query,
            debug: DebugContext::default(),
        }
    }

    // new_with
    #[must_use]
    pub fn new_with(path: &str, method: DeleteMethod) -> Self {
        Self {
            query: DeleteQuery::new(path, method),
            debug: DebugContext::default(),
        }
    }

    // execute
    pub fn execute(self, db: DataStoreRegistryLocal) -> Result<DeleteResponse, Error> {
        let executor = DeleteQueryExecutor::new(self.query, self.debug);
        executor.execute(db)
    }
}

///
/// DeleteQueryExecutor
///

pub struct DeleteQueryExecutor {
    query: DeleteQuery,
    debug: DebugContext,
    resolver: Resolver,
}

impl DeleteQueryExecutor {
    // new
    #[must_use]
    pub fn new(query: DeleteQuery, debug: DebugContext) -> Self {
        let resolver = Resolver::new(&query.path);

        Self {
            query,
            debug,
            resolver,
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // execute
    pub fn execute(&self, db: DataStoreRegistryLocal) -> Result<DeleteResponse, Error> {
        let query = &self.query;

        let keys = match &query.method {
            DeleteMethod::Undefined => {
                return Err(QueryError::DeleteError(DeleteError::Undefined))?;
            }
            DeleteMethod::One(key) => vec![key],
            DeleteMethod::Many(keys) => keys.iter().collect(),
        };

        // debug
        self.debug.println(&format!("delete: keys {keys:?}"));

        // get store
        let store_path = &self.resolver.store().map_err(QueryError::ResolverError)?;
        let store = db
            .with(|db| db.try_get_store(store_path))
            .map_err(QueryError::DbError)?;

        // execute for every different key
        let mut deleted_keys = Vec::new();
        for key in keys {
            let data_key = self
                .resolver
                .data_key(key)
                .map_err(QueryError::ResolverError)?;

            // remove returns DataValue but we ignore it for now
            // if the key is deleted then add it to the vec
            if store
                .with_borrow_mut(|store| store.remove(&data_key))
                .is_some()
            {
                deleted_keys.push(data_key);
            }
        }

        // debug
        self.debug
            .println(&format!("keys deleted: {deleted_keys:?}"));

        Ok(DeleteResponse(deleted_keys))
    }
}
