use crate::{
    Error,
    data::{
        DataError,
        executor::{DebugContext, ResolvedEntity, ResolvedSelector, with_resolver},
        query::{DeleteQuery, QueryError},
        response::{DeleteCollection, DeleteResponse, DeleteRow},
        store::{DataStoreRegistry, IndexStoreRegistry, SortKey},
    },
    deserialize,
    traits::EntityKind,
};
use std::collections::HashMap;

///
/// DeleteExecutor
///

pub struct DeleteExecutor {
    data_reg: DataStoreRegistry,
    index_reg: IndexStoreRegistry,
    debug: DebugContext,
}

impl DeleteExecutor {
    // new
    #[must_use]
    pub fn new(data_reg: DataStoreRegistry, index_reg: IndexStoreRegistry) -> Self {
        Self {
            data_reg,
            index_reg,
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
    pub fn execute<E: EntityKind>(self, query: DeleteQuery) -> Result<DeleteCollection, Error> {
        let res = self.execute_internal::<E>(query)?;

        Ok(res)
    }

    // execute_response
    // for when we have to return to the front end
    pub fn execute_response<E: EntityKind>(
        self,
        query: DeleteQuery,
    ) -> Result<DeleteResponse, Error> {
        let res = self.execute_internal::<E>(query)?;

        Ok(DeleteResponse(res.0))
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        &self,
        query: DeleteQuery,
    ) -> Result<DeleteCollection, DataError> {
        self.debug
            .println(&format!("query.delete: query is {query:?}"));

        // resolver
        let resolved = with_resolver(|r| r.entity(E::PATH))?;
        let selector = resolved.selector(&query.selector);
        let sort_keys: Vec<SortKey> = match selector {
            ResolvedSelector::One(key) => vec![key],
            ResolvedSelector::Many(keys) => keys,
            ResolvedSelector::Range(..) => {
                return Err(QueryError::SelectorNotSupported)?;
            }
        };

        // get store
        let store = self
            .data_reg
            .with(|db| db.try_get_store(resolved.store_path()))?;

        //
        // execute for every different key
        //
        let mut deleted_rows = Vec::new();

        for sk in sort_keys {
            if let Some(data_value) = store.with_borrow(|store| store.get(&sk)) {
                // Step 1: Deserialize the entity
                let entity: E = deserialize(&data_value.bytes)?;

                // Step 2: Extract field values
                let field_values = entity.key_values();

                // Step 3: Remove indexes
                self.remove_indexes(&resolved, &field_values)?;

                // Step 4: Delete the data row itself
                store.with_borrow_mut(|store| {
                    store.remove(&sk);
                });

                deleted_rows.push(DeleteRow::new(sk));
            }
        }

        // debug
        self.debug
            .println(&format!("query.delete: deleted keys {deleted_rows:?}"));

        Ok(DeleteCollection(deleted_rows))
    }

    // remove_indexes
    fn remove_indexes(
        &self,
        resolved: &ResolvedEntity,
        field_values: &HashMap<String, Option<String>>,
    ) -> Result<(), DataError> {
        for index in resolved.indexes() {
            // skip invalid index keys
            let Some(index_key) = resolved.build_index_key(index, field_values) else {
                continue;
            };

            let composite_key = resolved.composite_key(field_values);
            let index_store = self.index_reg.with(|ix| ix.try_get_store(&index.store))?;

            index_store.with_borrow_mut(|store| {
                if let Some(mut existing) = store.get(&index_key) {
                    existing.remove(&composite_key);

                    if existing.is_empty() {
                        store.remove(&index_key);
                    } else {
                        store.insert(index_key.clone(), existing);
                    }

                    self.debug.println(&format!(
                        "query.delete: removed index {index_key:?} - {composite_key:?}"
                    ));
                }

                Ok::<(), DataError>(())
            })?;
        }

        Ok(())
    }
}
