use crate::{
    Error,
    db::{DbLocal, types::DataKey},
    orm::traits::Entity,
    query::{
        DebugContext, QueryError, Resolver,
        delete::{DeleteError, DeleteResult},
    },
};
use candid::CandidType;
use serde::Serialize;
use std::{fmt::Display, marker::PhantomData};

///
/// DeleteBuilder
///

#[derive(Default)]
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
        Self {
            phantom: PhantomData,
        }
    }

    // one
    pub fn one<T: Display>(self, ck: &[T]) -> DeleteQuery<E> {
        let key = ck.iter().map(ToString::to_string).collect();
        let keys = vec![key];

        DeleteQuery::new(&keys)
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

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // one
    pub fn one<T: Display>(mut self, ck: &[T]) -> Result<DeleteExecutor<E>, Error> {
        let key: Vec<String> = ck.iter().map(ToString::to_string).collect();
        self.keys = vec![key];

        let executor = DeleteExecutor::new(self);

        Ok(executor)
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
    pub fn execute(&self, db: DbLocal) -> Result<DeleteResult, Error> {
        let mut keys_deleted = Vec::new();
        crate::ic::println!("delete: keys {:?}", &self.query.keys);

        for key in &self.query.keys {
            // If successful, push the key to results
            let res = self.execute_one(db, key).map_err(QueryError::DeleteError)?;

            keys_deleted.push(res);
        }

        self.query
            .debug
            .println(&format!("deleted keys {keys_deleted:?}"));

        Ok(DeleteResult::new(keys_deleted))
    }

    // execute_one
    fn execute_one(&self, db: DbLocal, ck: &[String]) -> Result<DataKey, DeleteError> {
        let key = self
            .resolver
            .data_key(ck)
            .map_err(DeleteError::ResolverError)?;

        let store = db
            .with(|db| db.try_get_store(E::STORE))
            .map_err(DeleteError::DbError)?;

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
