use crate::{
    db::{store::StoreLocal, types::DataKey},
    query::{
        delete::{DeleteError, DeleteResponse},
        DebugContext, Resolver,
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

///
/// DeleteBuilder
///

pub struct DeleteBuilder {
    path: String,
    debug: DebugContext,
}

impl DeleteBuilder {
    // new
    #[must_use]
    pub(crate) fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            debug: DebugContext::default(),
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // one
    pub fn one<T: Display>(self, ck: &[T]) -> Result<DeleteQuery, DeleteError> {
        let key: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let executor = DeleteQuery::new(self, vec![key]);

        Ok(executor)
    }
}

///
/// DeleteQuery
///
/// results : all the keys that have successfully been deleted
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct DeleteQuery {
    path: String,
    debug: DebugContext,
    keys: Vec<Vec<String>>,
}

impl DeleteQuery {
    // new
    #[must_use]
    fn new(builder: DeleteBuilder, keys: Vec<Vec<String>>) -> Self {
        Self {
            path: builder.path,
            debug: builder.debug,
            keys,
        }
    }

    // execute
    pub fn execute(self, store: StoreLocal) -> Result<DeleteResponse, DeleteError> {
        let executor = DeleteExecutor::new(self);

        executor.execute(store)
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
    pub fn execute(&self, store: StoreLocal) -> Result<DeleteResponse, DeleteError> {
        let mut results = Vec::new();
        crate::ic::println!("delete: keys {:?}", &self.query.keys);

        for key in &self.query.keys {
            // If successful, push the key to results
            let res = self.execute_one(store, key)?;

            results.push(res);
        }

        self.query
            .debug
            .println(&format!("deleted keys {results:?}"));

        Ok(DeleteResponse::new(results))
    }

    fn execute_one(&self, store: StoreLocal, key: &[String]) -> Result<DataKey, DeleteError> {
        // Attempt to remove the item from the store
        let data_key = self.resolver.data_key(key)?;
        //   let store_path = self.resolver.store()?;

        store.with_borrow_mut(|store| {
            if store.remove(&data_key).is_none() {
                crate::ic::println!("key {data_key:?} not found");
            }
        });

        Ok(data_key)
    }
}
