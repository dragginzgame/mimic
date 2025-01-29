use crate::{
    db::{store::StoreLocal, types::DataKey},
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
    pub fn execute(self, store: StoreLocal) -> Result<DeleteResponse, DeleteError> {
        let executor = EDeleteExecutor::new(self);

        executor.execute(store)
    }
}

///
/// EDeleteExecutor
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

    // execute_one
    fn execute_one(&self, store: StoreLocal, ck: &[String]) -> Result<DataKey, DeleteError> {
        // Attempt to remove the item from the store
        let key = self.resolver.data_key(ck)?;

        store.with_borrow_mut(|store| {
            store
                .remove(&key)
                .ok_or_else(|| DeleteError::KeyNotFound { key: key.clone() })
        })?;

        Ok(key)
    }
}
