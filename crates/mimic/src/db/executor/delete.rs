use crate::{
    Error,
    db::{
        DataError,
        executor::ResolvedSelector,
        query::{DeleteQuery, QueryError},
        response::{DeleteCollection, DeleteResponse, DeleteRow},
        store::{DataKey, DataStoreRegistry, IndexStoreRegistry},
    },
    debug,
    ops::{serialize::deserialize, traits::EntityKind},
};

///
/// DeleteExecutor
///

pub struct DeleteExecutor {
    data_registry: DataStoreRegistry,
    index_registry: IndexStoreRegistry,
    debug: bool,
}

impl DeleteExecutor {
    // new
    #[must_use]
    pub const fn new(data_registry: DataStoreRegistry, index_registry: IndexStoreRegistry) -> Self {
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
        debug!(self.debug, "query.delete: query is {query:?}");

        // resolver
        let resolved_selector = query.selector.resolve::<E>();
        let data_keys: Vec<DataKey> = match resolved_selector {
            ResolvedSelector::One(key) => vec![key],
            ResolvedSelector::Many(keys) => keys,
            ResolvedSelector::Range(..) => {
                return Err(QueryError::SelectorNotSupported)?;
            }
        };

        // get store
        let store = self.data_registry.with(|db| db.try_get_store(E::STORE))?;

        //
        // execute for every different key
        //

        let mut deleted_rows = Vec::new();

        for dk in data_keys {
            let Some(data_value) = store.with_borrow(|s| s.get(&dk)) else {
                continue;
            };

            // Step 1: Deserialize the entity and get values
            let entity: E = deserialize(&data_value.bytes)?;

            // Step 2: Remove indexes
            self.remove_indexes::<E>(entity)?;

            // Step 3: Delete the data row itself
            store.with_borrow_mut(|store| {
                store.remove(&dk);
            });

            deleted_rows.push(DeleteRow::new(dk.into()));
        }

        // debug
        debug!(self.debug, "query.delete: deleted keys {deleted_rows:?}");

        Ok(DeleteCollection(deleted_rows))
    }

    // remove_indexes
    fn remove_indexes<E: EntityKind>(&self, entity: E) -> Result<(), DataError> {
        let entity_key = entity.entity_key();

        for index in E::INDEXES {
            // resolve index key
            if let Some(index_key) = entity.index_key(index.fields) {
                // remove if found
                let index_store = self
                    .index_registry
                    .with(|ix| ix.try_get_store(index.store))?;

                index_store.with_borrow_mut(|store| {
                    store.remove_index_entry(&index_key, &entity_key);
                });
            }
        }

        Ok(())
    }
}
