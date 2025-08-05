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
    debug,
};
use icu::utils::time::now_secs;

///
/// SaveExecutor
///

#[derive(Clone, Copy, Debug)]
pub struct SaveExecutor {
    data_registry: DataStoreRegistryLocal,
    index_registry: IndexStoreRegistryLocal,
    debug: bool,
}

impl SaveExecutor {
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
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    // execute
    // serializes the save query to pass to execute_internal
    pub fn execute<E: EntityKind>(&self, query: SaveQuery) -> Result<Key, Error> {
        let bytes: E = deserialize(&query.bytes)?;
        let key = self.execute_internal::<E>(query.mode, bytes)?;

        Ok(key)
    }

    // create
    pub fn create<E: EntityKind>(&self, entity: E) -> Result<Key, Error> {
        let key = self.execute_internal::<E>(SaveMode::Create, entity)?;

        Ok(key)
    }

    // create_from_view
    pub fn create_from_view<E: EntityKind>(&self, view: E::View) -> Result<Key, Error>
    where
        E::View: Clone + Into<E>,
    {
        self.create(view.into())
    }

    // update
    pub fn update<E: EntityKind>(&self, entity: E) -> Result<Key, Error> {
        let key = self.execute_internal::<E>(SaveMode::Update, entity)?;

        Ok(key)
    }

    // replace
    pub fn replace<E: EntityKind>(&self, entity: E) -> Result<Key, Error> {
        let key = self.execute_internal::<E>(SaveMode::Replace, entity)?;

        Ok(key)
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(&self, mode: SaveMode, entity: E) -> Result<Key, DbError> {
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
                    return Ok(key);
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

        Ok(key)
    }

    // replace_indexes
    fn replace_indexes<E: EntityKind>(&self, old: Option<&E>, new: &E) -> Result<(), DbError> {
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
