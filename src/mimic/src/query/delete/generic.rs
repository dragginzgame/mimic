use crate::{
    orm::traits::Entity,
    query::{
        delete::{DeleteError, DeleteResponse},
        DebugContext, QueryError, Resolver,
    },
    store::{types::DataKey, StoreLocal},
    Error,
};
use candid::CandidType;
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};

///
/// DeleteBuilder
///

pub struct DeleteBuilder<E>
where
    E: Entity,
{
    debug: DebugContext,
    _phantom: PhantomData<E>,
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
    pub fn one<T: Display>(self, ck: &[T]) -> Result<DeleteQuery<E>, Error> {
        let key: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let executor = DeleteQuery::from_builder(self, vec![key]);

        Ok(executor)
    }
}

///
/// DeleteQuery
///
/// results : all the keys that have successfully been deleted
///

#[derive(CandidType, Debug, Serialize)]
pub struct DeleteQuery<E>
where
    E: Entity,
{
    debug: DebugContext,
    keys: Vec<Vec<String>>,
    _phantom: PhantomData<E>,
}

impl<E> DeleteQuery<E>
where
    E: Entity,
{
    // new
    #[must_use]
    const fn from_builder(builder: DeleteBuilder<E>, keys: Vec<Vec<String>>) -> Self {
        Self {
            debug: builder.debug,
            keys,
            _phantom: PhantomData,
        }
    }

    // execute
    pub fn execute(self, store: StoreLocal) -> Result<DeleteResponse, Error> {
        let executor = DeleteExecutor::new(self);

        executor.execute(store)
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
        let resolver = Resolver::new(E::PATH);

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
    fn execute_one(&self, store: StoreLocal, ck: &[String]) -> Result<DataKey, Error> {
        let key = self
            .resolver
            .data_key(ck)
            .map_err(DeleteError::ResolverError)
            .map_err(QueryError::DeleteError)?;

        // Attempt to remove the item from the store
        store.with_borrow_mut(|store| {
            store
                .remove(&key)
                .ok_or_else(|| DeleteError::KeyNotFound(key.clone()))
                .map_err(QueryError::DeleteError)
        })?;

        Ok(key)
    }
}
