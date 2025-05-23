use crate::{
    Error,
    db::{DbLocal, types::SortKey},
    query::{DebugContext, QueryError, Resolver},
    traits::Entity,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, marker::PhantomData};

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

#[derive(Debug, Default)]
pub struct DeleteBuilder<E>
where
    E: Entity,
{
    phantom: PhantomData<E>,
}

impl<E> DeleteBuilder<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // one
    pub fn one<T: Display>(self, ck: &[T]) -> DeleteQuery {
        let key = ck.iter().map(ToString::to_string).collect();

        DeleteQuery::new(E::PATH, DeleteMethod::One(key))
    }

    // many
    #[must_use]
    pub fn many<T: Display>(self, ck: &[Vec<T>]) -> DeleteQuery {
        let keys: Vec<Vec<String>> = ck
            .iter()
            .map(|inner_vec| inner_vec.iter().map(ToString::to_string).collect())
            .collect();

        DeleteQuery::new(E::PATH, DeleteMethod::Many(keys))
    }
}

///
/// DeleteQuery
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
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
    pub const fn debug(mut self) -> Self {
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
        let keys = match &self.query.method {
            DeleteMethod::One(key) => vec![key],
            DeleteMethod::Many(keys) => keys.iter().collect(),
        };

        // debug
        self.query.debug.println(&format!("delete: keys {keys:?}"));

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
        self.query
            .debug
            .println(&format!("keys deleted: {deleted_keys:?}"));

        Ok(DeleteResponse(deleted_keys))
    }
}

///
/// DeleteResponse
///

#[derive(CandidType, Debug, Deref, DerefMut, Serialize, Deserialize)]
pub struct DeleteResponse(Vec<SortKey>);
