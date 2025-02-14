use crate::{
    db::{types::DataKey, StoreLocal},
    query::{
        delete::{DeleteError, DeleteResponse},
        DebugContext, QueryError, Resolver,
    },
    Error,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

///
/// DeleteBuilderPath
///

pub struct DeleteBuilderPath {
    debug: DebugContext,
}

impl DeleteBuilderPath {
    // new
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
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
    pub fn one<T: Display>(self, ck: &[T]) -> Result<DeleteQueryPath, Error> {
        let key: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let executor = DeleteQueryPath::from_builder(self, vec![key]);

        Ok(executor)
    }
}

///
/// DeleteQueryPath
///
/// results : all the keys that have successfully been deleted
///

#[derive(CandidType, Debug, Default, Serialize, Deserialize)]
pub struct DeleteQueryPath {
    path: String,
    keys: Vec<Vec<String>>,
    debug: DebugContext,
}

impl DeleteQueryPath {
    // new
    #[must_use]
    pub fn new(path: &str, keys: &[Vec<String>]) -> Self {
        Self {
            path: path.to_string(),
            keys: keys.to_vec(),
            ..Default::default()
        }
    }

    // from_builder
    #[must_use]
    const fn from_builder(builder: DeleteBuilderPath, keys: Vec<Vec<String>>) -> Self {
        Self {
            path: String::new(),
            keys,
            debug: builder.debug,
        }
    }

    // path
    #[must_use]
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    // execute
    pub fn execute(self, store: StoreLocal) -> Result<DeleteResponse, Error> {
        let executor = DeleteExecutorPath::new(self);

        executor.execute(store)
    }
}

///
/// DeleteExecutorDyn
///

pub struct DeleteExecutorPath {
    query: DeleteQueryPath,
    resolver: Resolver,
}

impl DeleteExecutorPath {
    // new
    #[must_use]
    pub fn new(query: DeleteQueryPath) -> Self {
        let resolver = Resolver::new(&query.path);

        Self { query, resolver }
    }

    // execute
    pub fn execute(&self, store: StoreLocal) -> Result<DeleteResponse, Error> {
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
