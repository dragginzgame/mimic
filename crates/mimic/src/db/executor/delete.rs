use crate::{
    MimicError,
    core::traits::EntityKind,
    db::{
        DbError,
        query::{DeleteQuery, QueryError, QueryPlan, QueryShape},
        response::{DeleteCollection, DeleteResponse, DeleteRow},
        store::{DataKey, DataKeyRange, DataStoreRegistry, IndexStoreRegistry},
    },
    debug,
    serialize::deserialize,
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
    pub fn execute<E: EntityKind>(
        self,
        query: DeleteQuery,
    ) -> Result<DeleteCollection, MimicError> {
        let res = self.execute_internal::<E>(query)?;

        Ok(res)
    }

    // execute_response
    // for when we have to return to the front end
    pub fn execute_response<E: EntityKind>(
        self,
        query: DeleteQuery,
    ) -> Result<DeleteResponse, MimicError> {
        let res = self.execute_internal::<E>(query)?;

        Ok(DeleteResponse(res.0))
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        &self,
        query: DeleteQuery,
    ) -> Result<DeleteCollection, DbError> {
        debug!(self.debug, "query.delete: query is {query:?}");

        let resolved = query.selector.resolve();
        let plan = QueryPlan::from_resolved_selector(resolved, None);

        // get store
        let store = self.data_registry.with(|db| db.try_get_store(E::STORE))?;

        // resolver
        let data_keys: Vec<DataKey> = match plan.shape {
            QueryShape::Single(entity_key) => vec![E::build_data_key(&entity_key)],
            QueryShape::Many(entity_keys) => entity_keys
                .into_iter()
                .map(|key| E::build_data_key(&key))
                .collect(),
            QueryShape::Range(range) => {
                let data_range = range.to_data_key_range::<E>();

                store.with_borrow(|s| match data_range {
                    DataKeyRange::Inclusive(r) => s.range(r).map(|(k, _)| k).collect(),
                    DataKeyRange::Exclusive(r) => s.range(r).map(|(k, _)| k).collect(),
                    DataKeyRange::SkipFirstInclusive(r) => {
                        let mut it = s.range(r);
                        it.next();
                        it.map(|(k, _)| k).collect()
                    }
                    DataKeyRange::SkipFirstExclusive(r) => {
                        let mut it = s.range(r);
                        it.next();
                        it.map(|(k, _)| k).collect()
                    }
                })
            }
            QueryShape::FullScan => {
                return Err(QueryError::SelectorNotSupported)?; // Optional: implement full delete
            }
        };

        // execute for every different key
        let mut deleted_rows = Vec::new();
        for dk in data_keys {
            let Some(data_value) = store.with_borrow(|s| s.get(&dk)) else {
                continue;
            };

            // deserialize and remove indexes
            let entity: E = deserialize(&data_value.bytes)?;
            self.remove_indexes::<E>(entity)?;

            // delete
            store.with_borrow_mut(|s| {
                s.remove(&dk);
            });

            deleted_rows.push(DeleteRow::new(dk.into()));
        }

        // debug
        debug!(self.debug, "query.delete: deleted keys {deleted_rows:?}");

        Ok(DeleteCollection(deleted_rows))
    }

    // remove_indexes
    fn remove_indexes<E: EntityKind>(&self, entity: E) -> Result<(), DbError> {
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
