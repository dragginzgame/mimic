use crate::{
    Error,
    data::{
        DataError,
        executor::{DebugContext, EntityValue, ExecutorError, ResolvedEntity, with_resolver},
        query::{SaveMode, SaveQuery, SaveQueryTyped},
        response::{SaveCollection, SaveResponse, SaveRow},
        store::{DataStoreRegistry, DataValue, IndexStoreRegistry, IndexValue, Metadata},
    },
    serialize,
    traits::EntityKind,
    utils::time,
};
use std::collections::HashMap;

///
/// SaveExecutor
///

pub struct SaveExecutor {
    data: DataStoreRegistry,
    indexes: IndexStoreRegistry,
    debug: DebugContext,
}

impl SaveExecutor {
    // new
    #[must_use]
    pub fn new(data: DataStoreRegistry, indexes: IndexStoreRegistry) -> Self {
        Self {
            data,
            indexes,
            debug: DebugContext::default(),
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // execute
    pub fn execute<E: EntityKind>(
        &self,
        query: SaveQueryTyped<E>,
    ) -> Result<SaveCollection, Error> {
        let res = self.execute_internal::<E>(query)?;

        Ok(res)
    }

    // for when we have to return to the front end
    pub fn execute_response<E: EntityKind>(
        self,
        query: SaveQueryTyped<E>,
    ) -> Result<SaveResponse, Error> {
        let res = self.execute_internal::<E>(query)?;

        Ok(SaveResponse(res.0))
    }

    // execute_dyn
    pub fn execute_dyn<E: EntityKind>(&self, query: SaveQuery) -> Result<SaveCollection, Error> {
        let typed = SaveQueryTyped::new(query.mode, crate::deserialize::<E>(&query.bytes)?);

        self.execute(typed)
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
        crate::validate(&entity)?;

        // resolve - get schema data
        let resolved = with_resolver(|r| r.entity(E::PATH))?;
        let key_values = &entity.key_values();
        let sk = resolved.sort_key(key_values);
        let store = self
            .data
            .with(|data| data.try_get_store(resolved.store_path()))?;

        // debug
        self.debug.println(&format!("query.{mode}: {sk}"));

        //
        // match save mode
        // on Update and Replace compare old and new data
        //

        let now = time::now_secs();
        let old_result = store.with_borrow(|store| store.get(&sk));

        // did anything change?

        let (created, modified, old_ev) = match (mode, old_result) {
            (SaveMode::Create, Some(_)) => return Err(ExecutorError::KeyExists(sk.clone()))?,
            (SaveMode::Create, None) => (now, now, None),
            (SaveMode::Update, None) => return Err(ExecutorError::KeyNotFound(sk.clone()))?,
            (SaveMode::Replace, None) => (now, now, None),

            (SaveMode::Update, Some(old_dv)) | (SaveMode::Replace, Some(old_dv)) => {
                let old_ev: EntityValue<E> = old_dv.try_into()?;

                // no changes
                if entity == old_ev.entity {
                    self.debug
                        .println(&format!("query.{mode}: no changes for {sk}, skipping save"));

                    return Ok(SaveCollection(vec![SaveRow {
                        key: sk,
                        created: old_ev.metadata.created,
                        modified: old_ev.metadata.modified,
                    }]));
                }

                (old_ev.metadata.created, now, Some(old_ev))
            }
        };

        // update indexes
        let old_key_values = old_ev.as_ref().map(|ev| ev.entity.key_values());
        self.update_indexes(&resolved, old_key_values.as_ref(), key_values, mode)?;

        // prepare data value
        let value = DataValue {
            data: bytes,
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
    fn update_indexes(
        &self,
        resolved: &ResolvedEntity,
        old_values: Option<&HashMap<String, Option<String>>>,
        new_values: &HashMap<String, Option<String>>,
        mode: SaveMode,
    ) -> Result<(), DataError> {
        for index in resolved.indexes() {
            let index_store = self.indexes.with(|map| map.try_get_store(&index.store))?;

            // 🔁 Remove old index entry if applicable
            if let Some(old) = old_values {
                if let Some(old_key) = resolved.build_index_key(index, old) {
                    let old_ck = resolved.composite_key(old);

                    index_store.with_borrow_mut(|istore| {
                        if let Some(mut existing) = istore.get(&old_key) {
                            existing.remove(&old_ck);

                            if existing.is_empty() {
                                istore.remove(&old_key);
                            } else {
                                istore.insert(old_key.clone(), existing);
                            }

                            self.debug.println(&format!(
                                "query.{mode:?}: removed index {old_key:?} - {old_ck:?}"
                            ));
                        }

                        Ok::<(), DataError>(())
                    })?;
                }
            }

            // ✅ Insert new index entry
            if let Some(new_key) = resolved.build_index_key(index, new_values) {
                let new_ck = resolved.composite_key(new_values);

                index_store.with_borrow_mut(|istore| {
                    let index_value = match istore.get(&new_key) {
                        Some(existing) if index.unique => {
                            if !existing.contains(&new_ck) && !existing.is_empty() {
                                return Err(ExecutorError::IndexViolation(new_key.clone()));
                            }
                            IndexValue::from(vec![new_ck])
                        }
                        Some(mut existing) => {
                            existing.insert(new_ck.clone());
                            existing
                        }
                        None => IndexValue::from(vec![new_ck]),
                    };

                    istore.insert(new_key.clone(), index_value.clone());

                    self.debug.println(&format!(
                        "query.{mode:?}: added index {new_key:?} - {index_value:?}"
                    ));

                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}
