use crate::{
    Error,
    db::{
        DataError,
        executor::ExecutorError,
        query::{SaveMode, SaveQueryTyped},
        response::{EntityEntry, SaveCollection, SaveResponse, SaveRow},
        store::{DataEntry, DataStoreRegistry, IndexStoreRegistry, Metadata},
    },
    debug,
    ops::{serialize::serialize, traits::EntityKind, validate::validate},
    utils::time,
};

///
/// SaveExecutor
///

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
    ) -> Result<SaveCollection, Error> {
        let res = self.execute_internal(query)?;

        Ok(res)
    }

    // for when we have to return to the front end
    pub fn execute_response<E: EntityKind>(
        self,
        query: SaveQueryTyped<E>,
    ) -> Result<SaveResponse, Error> {
        let res = self.execute_internal(query)?;

        Ok(SaveResponse(res.0))
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        &self,
        query: SaveQueryTyped<E>,
    ) -> Result<SaveCollection, DataError> {
        let mode = query.mode;
        let entity = query.entity;
        let bytes = serialize(&entity)?;

        // validate
        validate(&entity)?;

        // resolve - get schema data
        let dk = entity.data_key();
        let store = self.data.with(|data| data.try_get_store(E::STORE))?;

        // debug
        debug!(self.debug, "query.{mode}: {} ({dk}) ", E::PATH);

        //
        // match save mode
        // on Update and Replace compare old and new data
        //

        let now = time::now_secs();
        let old_result = store.with_borrow(|store| store.get(&dk));

        // did anything change?

        let (created, modified, old) = match (mode, old_result) {
            (SaveMode::Create, Some(_)) => return Err(ExecutorError::KeyExists(dk))?,
            (SaveMode::Create | SaveMode::Replace, None) => (now, now, None),
            (SaveMode::Update, None) => return Err(ExecutorError::KeyNotFound(dk))?,

            (SaveMode::Update | SaveMode::Replace, Some(old_dv)) => {
                let old_ev: EntityEntry<E> = old_dv.try_into()?;

                // no changes
                if entity == old_ev.entity {
                    debug!(
                        self.debug,
                        "query.{mode}: no changes for {dk}, skipping save"
                    );

                    return Ok(SaveCollection(vec![SaveRow {
                        key: dk.into(),
                        created: old_ev.metadata.created,
                        modified: old_ev.metadata.modified,
                    }]));
                }

                (old_ev.metadata.created, now, Some(old_ev.entity))
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
            store.insert(dk.clone(), entry);
        });

        // return a collection
        Ok(SaveCollection(vec![SaveRow {
            key: dk.into(),
            created,
            modified,
        }]))
    }

    // update_indexes
    fn update_indexes<E: EntityKind>(&self, old: Option<&E>, new: &E) -> Result<(), DataError> {
        for index in E::INDEXES {
            let index_store = self.indexes.with(|map| map.try_get_store(index.store))?;

            // ✅ Insert new index entry first - fail early if conflict
            if let Some(new_index_key) = new.index_key(index.fields) {
                index_store.with_borrow_mut(|store| {
                    store.insert_index_entry(index, new_index_key.clone(), new.entity_key())?;

                    Ok::<_, DataError>(())
                })?;
            }

            // 🔁 Remove old index value (if present and resolvable)
            if let Some(old) = old {
                if let Some(old_index_key) = old.index_key(index.fields) {
                    index_store.with_borrow_mut(|store| {
                        store.remove_index_entry(&old_index_key, &old.entity_key());
                    });
                }
            }
        }

        Ok(())
    }
}
