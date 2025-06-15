use crate::{
    Error,
    db::{
        DataError,
        executor::DebugContext,
        query::{DeleteQuery, QueryError},
        response::{DeleteCollection, DeleteResponse, DeleteRow},
        store::{DataStoreRegistry, IndexStoreRegistry},
        types::{IndexKey, ResolvedSelector, SortKey},
    },
    def::{deserialize, traits::EntityKind},
};

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
        let resolved_selector = query.selector.resolve::<E>();
        let sort_keys: Vec<SortKey> = match resolved_selector {
            ResolvedSelector::One(key) => vec![key],
            ResolvedSelector::Many(keys) => keys,
            ResolvedSelector::Range(..) => {
                return Err(QueryError::SelectorNotSupported)?;
            }
        };

        // get store
        let store = self.data_reg.with(|db| db.try_get_store(E::STORE))?;

        //
        // execute for every different key
        //
        let mut deleted_rows = Vec::new();

        for sk in sort_keys {
            let Some(data_value) = store.with_borrow(|s| s.get(&sk)) else {
                continue;
            };

            // Step 1: Deserialize the entity and get values
            let entity: E = deserialize(&data_value.bytes)?;

            // Step 2: Remove indexes
            self.remove_indexes::<E>(entity)?;

            // Step 3: Delete the data row itself
            store.with_borrow_mut(|store| {
                store.remove(&sk);
            });

            deleted_rows.push(DeleteRow::new(sk));
        }

        // debug
        self.debug
            .println(&format!("query.delete: deleted keys {deleted_rows:?}"));

        Ok(DeleteCollection(deleted_rows))
    }

    // remove_indexes
    fn remove_indexes<E: EntityKind>(&self, entity: E) -> Result<(), DataError> {
        let field_values = entity.values();
        let entity_key = entity.key();

        for index in E::INDEXES {
            // Gather all the field values required by the index
            let mut key_parts = Vec::new();
            let mut all_fields_present = true;

            for field in index.fields {
                match field_values.get(field) {
                    Some(value) => key_parts.push(value.clone()),
                    None => {
                        all_fields_present = false;
                        break;
                    }
                }
            }

            if !all_fields_present {
                continue;
            }

            let index_store = self.index_reg.with(|ix| ix.try_get_store(&index.store))?;
            let index_key = IndexKey::new(E::ID, index.fields, key_parts);

            index_store.with_borrow_mut(|store| {
                if let Some(mut existing) = store.get(&index_key) {
                    existing.remove(&entity_key);

                    if existing.is_empty() {
                        store.remove(&index_key);
                    } else {
                        store.insert(index_key.clone(), existing);
                    }

                    self.debug.println(&format!(
                        "query.delete: removed value {entity_key} from index {index_key:?}"
                    ));
                }
                Ok::<(), DataError>(())
            })?;
        }

        Ok(())
    }
}
