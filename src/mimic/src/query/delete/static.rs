use crate::{
    db::{types::DataKey, Db},
    orm::traits::Entity,
    query::{
        delete::{DeleteError, DeleteResponse},
        DebugContext, Resolver,
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, marker::PhantomData};

///
/// EDeleteBuilder
///

pub struct EDeleteBuilder<E>
where
    E: Entity,
{
    debug: DebugContext,
    _phantom: PhantomData<E>,
}

impl<E> EDeleteBuilder<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            debug: DebugContext::default(),
            _phantom: PhantomData,
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // one
    pub fn one<T: Display>(self, ck: &[T]) -> Result<EDeleteQuery<E>, DeleteError> {
        let key: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let executor = EDeleteQuery::new(self, vec![key]);

        Ok(executor)
    }
}

///
/// EDeleteQuery
///
/// results : all the keys that have successfully been deleted
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct EDeleteQuery<E>
where
    E: Entity,
{
    debug: DebugContext,
    keys: Vec<Vec<String>>,
    _phantom: PhantomData<E>,
}

impl<E> EDeleteQuery<E>
where
    E: Entity,
{
    // new
    #[must_use]
    const fn new(builder: EDeleteBuilder<E>, keys: Vec<Vec<String>>) -> Self {
        Self {
            debug: builder.debug,
            keys,
            _phantom: PhantomData,
        }
    }

    // execute
    pub fn execute(self, db: &Db) -> Result<DeleteResponse, DeleteError> {
        let executor = EDeleteExecutor::new(self);

        executor.execute(db)
    }
}

///
/// DeleteExecutor
///

pub struct EDeleteExecutor<E>
where
    E: Entity,
{
    query: EDeleteQuery<E>,
    resolver: Resolver,
}

impl<E> EDeleteExecutor<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new(query: EDeleteQuery<E>) -> Self {
        let resolver = Resolver::new(E::PATH);

        Self { query, resolver }
    }

    // execute
    pub fn execute(&self, db: &Db) -> Result<DeleteResponse, DeleteError> {
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

        Ok(DeleteResponse::new(results))
    }

    // execute_one
    fn execute_one(&self, db: &Db, key: &[String]) -> Result<DataKey, DeleteError> {
        // Attempt to remove the item from the store
        let data_key = self.resolver.data_key(key)?;
        let store_path = self.resolver.store()?;

        db.with_store_mut(&store_path, |store| {
            if store.remove(&data_key).is_none() {
                crate::ic::println!("key {data_key:?} not found");
            }

            Ok(())
        })
        .map_err(DeleteError::from)?;

        Ok(data_key)
    }
}
