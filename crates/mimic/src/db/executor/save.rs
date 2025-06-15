use crate::{
    Error,
    db::{
        DataError,
        executor::{ExecutorError, resolve_index_key},
        query::{SaveMode, SaveQueryTyped},
        response::{SaveCollection, SaveResponse, SaveRow},
        store::{DataStoreRegistry, IndexStoreRegistry},
        types::{DataValue, EntityValue, Metadata},
    },
    debug,
    def::{serialize, traits::EntityKind},
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
    pub fn new(data: DataStoreRegistry, indexes: IndexStoreRegistry) -> Self {
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
        crate::def::validate(&entity)?;

        // resolve - get schema data
        let sk = entity.sort_key();
        let store = self.data.with(|data| data.try_get_store(E::STORE))?;

        // debug
        debug!(self.debug, "query.{mode}: {} ({sk}) ", E::PATH);

        //
        // match save mode
        // on Update and Replace compare old and new data
        //

        let now = time::now_secs();
        let old_result = store.with_borrow(|store| store.get(&sk));

        // did anything change?

        let (created, modified, old) = match (mode, old_result) {
            (SaveMode::Create, Some(_)) => return Err(ExecutorError::KeyExists(sk))?,
            (SaveMode::Create | SaveMode::Replace, None) => (now, now, None),
            (SaveMode::Update, None) => return Err(ExecutorError::KeyNotFound(sk))?,

            (SaveMode::Update | SaveMode::Replace, Some(old_dv)) => {
                let old_ev: EntityValue<E> = old_dv.try_into()?;

                // no changes
                if entity == old_ev.entity {
                    debug!(
                        self.debug,
                        "query.{mode}: no changes for {sk}, skipping save"
                    );

                    return Ok(SaveCollection(vec![SaveRow {
                        key: sk,
                        created: old_ev.metadata.created,
                        modified: old_ev.metadata.modified,
                    }]));
                }

                (old_ev.metadata.created, now, Some(old_ev.entity))
            }
        };

        // update indexes
        self.update_indexes(old.as_ref(), &entity)?;

        // prepare data value
        let value = DataValue {
            bytes,
            path: E::path(),
            metadata: Metadata { created, modified },
        };

        // insert data row
        store.with_borrow_mut(|store| {
            store.insert(sk.clone(), value);
        });

        // return a collection
        Ok(SaveCollection(vec![SaveRow {
            key: sk,
            created,
            modified,
        }]))
    }

    // update_indexes
    fn update_indexes<E: EntityKind>(&self, old: Option<&E>, new: &E) -> Result<(), DataError> {
        for index in E::INDEXES {
            let index_store = self.indexes.with(|map| map.try_get_store(index.store))?;

            // 🔁 Remove old index value (if present and resolvable)
            if let Some(old) = old {
                if let Some(old_index_key) = resolve_index_key::<E>(index.fields, &old.values()) {
                    index_store.with_borrow_mut(|store| {
                        store.remove_index_value(index, &old_index_key, &old.key());
                    });
                }
            }

            // ✅ Insert new index entry
            if let Some(new_index_key) = resolve_index_key::<E>(index.fields, &new.values()) {
                index_store.with_borrow_mut(|store| {
                    store.insert_index_value(index, new_index_key, new.key());
                });
            }
        }

        Ok(())
    }
}
