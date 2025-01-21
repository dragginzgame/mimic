use crate::db::{
    query::{DebugContext, Error as QueryError, Resolver},
    types::DataKey,
    Db,
};
use crate::orm::traits::Entity;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{fmt::Display, marker::PhantomData};

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Db { source: crate::db::db::Error },

    #[snafu(transparent)]
    Resolver { source: super::resolver::Error },
}

///
/// DeleteBuilder
///

pub struct DeleteBuilder<E>
where
    E: Entity,
{
    debug: DebugContext,
    phantom: PhantomData<E>,
}

impl<E> DeleteBuilder<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            debug: DebugContext::default(),
            phantom: PhantomData,
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // one
    pub fn one<T: Display>(self, ck: &[T]) -> Result<DeleteQuery<E>, QueryError> {
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

pub struct DeleteQuery<E>
where
    E: Entity,
{
    debug: DebugContext,
    keys: Vec<Vec<String>>,
    phantom: PhantomData<E>,
}

impl<E> DeleteQuery<E>
where
    E: Entity,
{
    // new
    #[must_use]
    const fn new(prev: DeleteBuilder<E>, keys: Vec<Vec<String>>) -> Self {
        Self {
            debug: prev.debug,
            keys,
            phantom: PhantomData,
        }
    }

    // execute
    pub fn execute(self, db: &Db) -> Result<DeleteResult, QueryError> {
        let executor = DeleteExecutor::new(self);

        executor.execute(db)
    }
}

///
/// DeleteExecutor
///

pub struct DeleteExecutor<E>
where
    E: Entity,
{
    query: DeleteQuery<E>,
    resolver: Resolver,
}

impl<E> DeleteExecutor<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new(query: DeleteQuery<E>) -> Self {
        Self {
            query,
            resolver: Resolver::new(&E::path()),
        }
    }

    // execute
    fn execute(&self, db: &Db) -> Result<DeleteResult, QueryError> {
        let mut results = Vec::new();
        crate::ic::println!("delete: keys {:?}", &self.query.keys);

        for key in &self.query.keys {
            // If successful, push the key to results
            let res = self.execute_one(db, key)?;

            results.push(res);
        }

        self.query
            .debug
            .println(&format!("deleted keys {results:?}"));

        Ok(DeleteResult::new(results))
    }

    fn execute_one(&self, db: &Db, key: &[String]) -> Result<DataKey, Error> {
        // Attempt to remove the item from the store
        let data_key = self.resolver.data_key(key)?;
        let store_path = self.resolver.store()?;

        db.with_store_mut(&store_path, |store| {
            if store.remove(&data_key).is_none() {
                crate::ic::println!("key {data_key:?} not found");
            }

            Ok(())
        })?;

        Ok(data_key)
    }
}

///
/// DeleteResult
///
/// results : all the keys that have successfully been deleted
///

pub struct DeleteResult {
    results: Vec<DataKey>,
}

impl DeleteResult {
    // new
    const fn new(results: Vec<DataKey>) -> Self {
        Self { results }
    }

    // keys
    pub fn keys(self) -> Result<Vec<DataKey>, QueryError> {
        Ok(self.results)
    }
}
