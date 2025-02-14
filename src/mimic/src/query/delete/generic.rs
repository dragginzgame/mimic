use crate::{
    db::{types::DataKey, StoreLocal},
    orm::traits::Entity,
    query::{
        delete::{DeleteError, DeleteResult},
        DebugContext, QueryError, Resolver,
    },
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

#[derive(CandidType, Debug, Default, Serialize)]
pub struct DeleteQuery<E>
where
    E: Entity,
{
    keys: Vec<Vec<String>>,
    debug: DebugContext,
    _phantom: PhantomData<E>,
}

impl<E> DeleteQuery<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new(keys: &[Vec<String>]) -> Self {
        Self {
            keys: keys.to_vec(),
            ..Default::default()
        }
    }

    // from_builder
    #[must_use]
    const fn from_builder(builder: DeleteBuilder<E>, keys: Vec<Vec<String>>) -> Self {
        Self {
            keys,
            debug: builder.debug,
            _phantom: PhantomData,
        }
    }

    // execute
    pub fn execute(self, store: StoreLocal) -> Result<DeleteResult, Error> {
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
    pub fn execute(&self, store: StoreLocal) -> Result<DeleteResult, Error> {
        let mut keys_deleted = Vec::new();
        crate::ic::println!("delete: keys {:?}", &self.query.keys);

        for key in &self.query.keys {
            // If successful, push the key to results
            let res = self
                .execute_one(store, key)
                .map_err(QueryError::DeleteError)?;

            keys_deleted.push(res);
        }

        self.query
            .debug
            .println(&format!("deleted keys {keys_deleted:?}"));

        Ok(DeleteResult::new(keys_deleted))
    }

    // execute_one
    fn execute_one(&self, store: StoreLocal, ck: &[String]) -> Result<DataKey, DeleteError> {
        let key = self
            .resolver
            .data_key(ck)
            .map_err(DeleteError::ResolverError)?;

        // Attempt to remove the item from the store
        store.with_borrow_mut(|store| {
            store
                .remove(&key)
                .ok_or_else(|| DeleteError::KeyNotFound(key.clone()))?;

            Ok::<_, DeleteError>(())
        })?;

        Ok(key)
    }
}
