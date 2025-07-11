use crate::{
    MimicError,
    common::utils::time,
    core::{traits::EntityKind, validate::validate},
    db::{
        DbError,
        executor::ExecutorError,
        query::{SaveMode, SaveQueryTyped},
        response::{EntityEntry, SaveCollection, SaveResponse, SaveRow},
        store::{DataEntry, DataKey, DataStoreRegistry, IndexKey, IndexStoreRegistry, Metadata},
    },
    debug,
    serialize::serialize,
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
    pub fn execute<E: EntityKind>(
        &self,
        query: SaveQueryTyped<E>,
    ) -> Result<SaveCollection, MimicError> {
        let res = self.execute_internal(query)?;

        Ok(res)
    }

    // for when we have to return to the front end
    pub fn execute_response<E: EntityKind>(
        self,
        query: SaveQueryTyped<E>,
    ) -> Result<SaveResponse, MimicError> {
        let res = self.execute_internal(query)?;

        Ok(SaveResponse(res.0))
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        &self,
        query: SaveQueryTyped<E>,
    ) -> Result<SaveCollection, DbError> {
        let mode = query.mode;
        let entity = query.entity;
        let bytes = serialize(&entity)?;

        // validate
        validate(&entity)?;

        // resolve - get schema data
        let key = entity.key();
        let store = self.data.with(|data| data.try_get_store(E::STORE))?;

        // debug
        debug!(self.debug, "query.{mode}: {} ({key}) ", E::PATH);

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

                    return Ok(SaveCollection(vec![SaveRow {
                        key: data_key.into(),
                        created: old_entity_value.metadata.created,
                        modified: old_entity_value.metadata.modified,
                    }]));
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
            path: E::path(),
            metadata: Metadata { created, modified },
        };

        // insert data row
        store.with_borrow_mut(|store| {
            store.insert(data_key.clone(), entry);
        });

        // return a collection
        Ok(SaveCollection(vec![SaveRow {
            key: data_key.into(),
            created,
            modified,
        }]))
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
