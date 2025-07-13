use crate::{
    MimicError,
    common::utils::time,
    core::{Key, traits::EntityKind, validate::validate},
    db::{
        DbError,
        executor::ExecutorError,
        query::{SaveMode, SaveQuery},
        response::EntityEntry,
        store::{DataEntry, DataKey, DataStoreRegistry, IndexKey, IndexStoreRegistry, Metadata},
    },
    debug,
    serialize::{deserialize, serialize},
};

///
/// SaveExecutor
///

#[derive(Clone, Copy, Debug)]
pub struct SaveExecutor {
    data: DataStoreRegistry,
    indexes: IndexStoreRegistry,
    debug: bool,
}

impl SaveExecutor {
    // new
    #[must_use]
    pub const fn new(data: DataStoreRegistry, indexes: IndexStoreRegistry) -> Self {
        Self {
            data,
            indexes,
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
    pub fn execute<E: EntityKind>(&self, query: SaveQuery) -> Result<Key, MimicError> {
        let bytes: E = deserialize(&query.bytes)?;
        let key = self.execute_internal::<E>(query.mode, bytes)?;

        Ok(key)
    }

    // create
    pub fn create<E: EntityKind>(&self, entity: E) -> Result<Key, MimicError> {
        let key = self.execute_internal::<E>(SaveMode::Create, entity)?;

        Ok(key)
    }

    // create_from_view
    pub fn create_from_view<E: EntityKind>(&self, view: &E::View) -> Result<Key, MimicError>
    where
        E::View: Clone + Into<E>,
    {
        self.create(view.clone().into())
    }

    // update
    pub fn update<E: EntityKind>(&self, entity: E) -> Result<Key, MimicError> {
        let key = self.execute_internal::<E>(SaveMode::Update, entity)?;

        Ok(key)
    }

    // replace
    pub fn replace<E: EntityKind>(&self, entity: E) -> Result<Key, MimicError> {
        let key = self.execute_internal::<E>(SaveMode::Replace, entity)?;

        Ok(key)
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(&self, mode: SaveMode, entity: E) -> Result<Key, DbError> {
        let key = entity.key();
        let store = self.data.with(|data| data.try_get_store(E::STORE))?;
        let bytes = serialize(&entity)?;

        // validate
        validate(&entity)?;

        // debug
        debug!(self.debug, "query.{mode}: {} ({key:?}) ", E::PATH);

        //
        // match save mode
        // on Update and Replace compare old and new data
        //

        let now = time::now_secs();
        let data_key = DataKey::with_entity::<E>(key);
        let old_result = store.with_borrow(|store| store.get(&data_key));

        // did anything change?
        let (created, modified, old) = match (mode, old_result) {
            (SaveMode::Create, Some(_)) => return Err(ExecutorError::KeyExists(data_key))?,
            (SaveMode::Create | SaveMode::Replace, None) => (now, now, None),
            (SaveMode::Update, None) => return Err(ExecutorError::KeyNotFound(data_key))?,

            (SaveMode::Update | SaveMode::Replace, Some(old_data_value)) => {
                let old_entity_value: EntityEntry<E> = old_data_value.try_into()?;

                // no changes
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
        };

        // update indexes, fail if there are any unique violations
        self.update_indexes(old.as_ref(), &entity)?;

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

    // update_indexes
    fn update_indexes<E: EntityKind>(&self, old: Option<&E>, new: &E) -> Result<(), DbError> {
        for index in E::INDEXES {
            let index_store = self.indexes.with(|map| map.try_get_store(index.store))?;

            // ✅ Insert new index entry first - fail early if conflict
            if let Some(new_index_key) = IndexKey::build(new, index.fields) {
                index_store.with_borrow_mut(|store| {
                    store.insert_index_entry(index, new_index_key.clone(), new.key())?;

                    Ok::<_, DbError>(())
                })?;
            }

            // 🔁 Remove old index value (if present)
            if let Some(old) = old {
                if let Some(old_index_key) = IndexKey::build(old, index.fields) {
                    index_store.with_borrow_mut(|store| {
                        store.remove_index_entry(&old_index_key, &old.key());
                    });
                }
            }
        }

        Ok(())
    }
}
