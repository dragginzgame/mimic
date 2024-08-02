use crate::{DebugContext, Error, Resolver};
use db::{DataKey, Db};
use orm::traits::Entity;
use std::{fmt::Display, marker::PhantomData};

///
/// DeleteBuilder
///

pub struct DeleteBuilder<'a, E>
where
    E: Entity,
{
    db: &'a Db,
    debug: DebugContext,
    phantom: PhantomData<E>,
}

impl<'a, E> DeleteBuilder<'a, E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new(db: &'a Db) -> Self {
        Self {
            db,
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
    pub fn one<T: Display>(self, ck: &[T]) -> Result<DeleteBuilderResult, Error> {
        let key: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let executor = DeleteBuilderExecutor::new(self, vec![key]);

        executor.execute()
    }
}

///
/// DeleteBuilderExecutor
/// (final stage)
///
/// results : all the keys that have successfully been deleted
///

pub struct DeleteBuilderExecutor<'a, E>
where
    E: Entity,
{
    db: &'a Db,
    debug: DebugContext,
    resolver: Resolver,
    keys: Vec<Vec<String>>,
    phantom: PhantomData<E>,
}

impl<'a, E> DeleteBuilderExecutor<'a, E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new(prev: DeleteBuilder<'a, E>, keys: Vec<Vec<String>>) -> Self {
        Self {
            db: prev.db,
            debug: prev.debug,
            resolver: Resolver::new(&E::path()),
            keys,
            phantom: PhantomData,
        }
    }

    // execute
    pub fn execute(&self) -> Result<DeleteBuilderResult, Error> {
        let mut results = Vec::new();
        lib_ic::println!("delete: keys {:?}", &self.keys);

        for key in &self.keys {
            // Attempt to remove the item from the store
            let data_key = self.resolver.data_key(key)?;
            let store_path = self.resolver.store()?;

            self.db.with_store_mut(&store_path, |store| {
                if store.remove(&data_key).is_none() {
                    lib_ic::println!("key {data_key:?} not found");
                }

                Ok(())
            })?;

            // If successful, push the key to results
            results.push(data_key.clone());
        }

        self.debug.println(&format!("deleted keys {results:?}"));

        Ok(DeleteBuilderResult::new(results))
    }
}

///
/// DeleteBuilderResult
///
/// results : all the keys that have successfully been deleted
///

pub struct DeleteBuilderResult {
    results: Vec<DataKey>,
}

impl DeleteBuilderResult {
    const fn new(results: Vec<DataKey>) -> Self {
        Self { results }
    }

    // keys
    pub fn keys(self) -> Result<Vec<DataKey>, Error> {
        Ok(self.results)
    }
}
