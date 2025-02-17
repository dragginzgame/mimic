use crate::{
    Error,
    db::{DbLocal, StoreLocal, types::DataKey},
    query::{
        DebugContext, QueryError, Resolver,
        delete::{DeleteError, DeleteResponse},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

///
/// DeleteBuilderDyn
///

#[derive(Default)]
pub struct DeleteBuilderDyn {
    path: String,
}

impl DeleteBuilderDyn {
    // new
    #[must_use]
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    // one
    pub fn one<T: Display>(self, ck: &[T]) -> DeleteQueryDyn {
        let key = ck.iter().map(ToString::to_string).collect();
        let keys = vec![key];

        DeleteQueryDyn::new(&self.path, &keys)
    }
}

///
/// DeleteQueryDyn
/// no builder needed as its simple
///

#[derive(CandidType, Debug, Default, Serialize, Deserialize)]
pub struct DeleteQueryDyn {
    path: String,
    keys: Vec<Vec<String>>,
    debug: DebugContext,
}

impl DeleteQueryDyn {
    // new
    #[must_use]
    pub fn new(path: &str, keys: &[Vec<String>]) -> Self {
        Self {
            path: path.to_string(),
            keys: keys.to_vec(),
            ..Default::default()
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // path
    #[must_use]
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }
}

///
/// DeleteExecutorDyn
///

pub struct DeleteExecutorDyn {
    query: DeleteQueryDyn,
    resolver: Resolver,
}

impl DeleteExecutorDyn {
    // new
    #[must_use]
    pub fn new(query: DeleteQueryDyn) -> Self {
        let resolver = Resolver::new(&query.path);

        Self { query, resolver }
    }

    // execute
    pub fn execute(&self, db: DbLocal) -> Result<DeleteResponse, Error> {
        let mut keys_deleted = Vec::new();
        crate::ic::println!("delete: keys {:?}", &self.query.keys);

        // get store
        let store_path = &self
            .resolver
            .store()
            .map_err(DeleteError::ResolverError)
            .map_err(QueryError::DeleteError)?;
        let store = db.with(|db| db.try_get_store(store_path))?;

        // execute for every different key
        for key in &self.query.keys {
            // If successful, push the key to results
            let res = self.execute_one(store, key)?;

            keys_deleted.push(res);
        }

        self.query
            .debug
            .println(&format!("deleted keys {keys_deleted:?}"));

        let res = DeleteResponse::new(keys_deleted);

        Ok(res)
    }

    // execute_one
    fn execute_one(&self, store: StoreLocal, key: &[String]) -> Result<DataKey, Error> {
        // Attempt to remove the item from the store
        let data_key = self
            .resolver
            .data_key(key)
            .map_err(DeleteError::ResolverError)
            .map_err(QueryError::DeleteError)?;
        //   let store_path = self.resolver.store()?;

        store.with_borrow_mut(|store| {
            if store.remove(&data_key).is_none() {
                crate::ic::println!("key {data_key:?} not found");
            }

            Ok::<_, Error>(())
        })?;

        Ok(data_key)
    }
}
