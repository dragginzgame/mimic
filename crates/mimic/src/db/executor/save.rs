use crate::{
    Error,
    core::{
        Key, deserialize, serialize,
        traits::{EntityKind, Path},
        validate::validate,
    },
    db::{
        DbError,
        executor::ExecutorError,
        query::{SaveMode, SaveQuery},
        response::EntityEntry,
        store::{DataEntry, DataKey, DataStoreRegistryLocal, IndexStoreRegistryLocal, Metadata},
    },
};
use icu::{debug, utils::time::now_secs};
use std::marker::PhantomData;

///
/// SaveExecutor
///

#[derive(Clone, Copy)]
pub struct SaveExecutor<E: EntityKind> {
    data_registry: DataStoreRegistryLocal,
    index_registry: IndexStoreRegistryLocal,
    debug: bool,
    _marker: PhantomData<E>,
}

impl<E: EntityKind> SaveExecutor<E> {
    // new
    #[must_use]
    pub const fn new(
        data_registry: DataStoreRegistryLocal,
        index_registry: IndexStoreRegistryLocal,
    ) -> Self {
        Self {
            data_registry,
            index_registry,
            debug: false,
            _marker: PhantomData,
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    ///
    /// EXECUTION METHODS
    ///

    // response
    // a specific response used by the automated query endpoints that
    // we will improve int he future
    pub fn response(&self, query: &SaveQuery) -> Result<Key, Error> {
        let bytes: E = deserialize(&query.bytes)?;
        let key = self.save_entity(query.mode, bytes)?.key();

        Ok(key)
    }

    pub fn create(&self, entity: E) -> Result<E, Error> {
        let entity = self.save_entity(SaveMode::Create, entity)?;

        Ok(entity)
    }

    pub fn create_view<V>(&self, view: E::View) -> Result<E::View, Error> {
        let entity = E::from_view(view);
        let saved_view = self.create(entity)?.to_view();

        Ok(saved_view)
    }

    pub fn update(&self, entity: E) -> Result<E, Error> {
        let entity = self.save_entity(SaveMode::Update, entity)?;

        Ok(entity)
    }

    pub fn update_view<V>(&self, view: E::View) -> Result<E::View, Error> {
        let entity = E::from_view(view);
        let saved_view = self.update(entity)?.to_view();

        Ok(saved_view)
    }

    pub fn replace(&self, entity: E) -> Result<E, Error> {
        let entity = self.save_entity(SaveMode::Replace, entity)?;

        Ok(entity)
    }

    pub fn replace_view<V>(&self, view: E::View) -> Result<E::View, Error> {
        let entity = E::from_view(view);
        let saved_view = self.replace(entity)?.to_view();

        Ok(saved_view)
    }

    // execute
    // serializes the save query to pass to save_entity
    pub fn execute(&self, query: &SaveQuery) -> Result<E, Error> {
        let bytes: E = deserialize(&query.bytes)?;
        let entity = self.save_entity(query.mode, bytes)?;

        Ok(entity)
    }

    // save_entity
    fn save_entity(&self, mode: SaveMode, entity: E) -> Result<E, DbError> {
        let key = entity.key();
        let store = self
            .data_registry
            .with(|reg| reg.try_get_store(E::Store::PATH))?;

        // validate
        validate(&entity)?;

        // debug
        debug!(self.debug, "query.{mode}: {} ({key:?}) ", E::PATH);

        //
        // match save mode
        // on Update and Replace compare old and new data
        //

        let now = now_secs();
        let data_key = DataKey::new::<E>(key);
        let old_result = store.with_borrow(|store| store.get(&data_key));

        // did anything change?
        let (created, modified, old) = match (mode, old_result) {
            (SaveMode::Create | SaveMode::Replace, None) => (now, now, None),

            (SaveMode::Update | SaveMode::Replace, Some(old_data_value)) => {
                let old_entity_value: EntityEntry<E> = old_data_value.try_into()?;

                if entity == old_entity_value.entity {
                    debug!(
                        self.debug,
                        "query.{mode}: no changes for {data_key}, skipping save"
                    );
                    return Ok(entity);
                }

                (
                    old_entity_value.metadata.created,
                    now,
                    Some(old_entity_value.entity),
                )
            }

            // invalid
            (SaveMode::Create, Some(_)) => return Err(ExecutorError::KeyExists(data_key))?,
            (SaveMode::Update, None) => return Err(ExecutorError::KeyNotFound(data_key))?,
        };

        // now we can serialize
        let bytes = serialize(&entity)?;

        // replace indexes, fail if there are any unique violations
        self.replace_indexes(old.as_ref(), &entity)?;

        // prepare data
        let entry = DataEntry {
            bytes,
            metadata: Metadata { created, modified },
        };

        // insert data row
        store.with_borrow_mut(|store| {
            store.insert(data_key.clone(), entry);
        });

        Ok(entity)
    }

    // replace_indexes
    fn replace_indexes(&self, old: Option<&E>, new: &E) -> Result<(), DbError> {
        for index in E::INDEXES {
            let store = self
                .index_registry
                .with(|reg| reg.try_get_store(index.store))?;

            store.with_borrow_mut(|s| {
                // remove first
                if let Some(old) = old {
                    s.remove_index_entry(old, index);
                }
                s.insert_index_entry(new, index)?;

                Ok::<(), DbError>(())
            })?;
        }

        Ok(())
    }
}
