use crate::{
    Error,
    data::{
        DataError,
        executor::{DebugContext, ResolvedSelector, with_resolver},
        query::{DeleteQuery, QueryError},
        response::DeleteResponse,
        store::{DataStoreRegistry, IndexStoreRegistry, SortKey},
    },
    deserialize,
    traits::EntityKind,
};

///
/// DeleteExecutor
///

pub struct DeleteExecutor {
    data: DataStoreRegistry,
    indexes: IndexStoreRegistry,
    debug: DebugContext,
}

impl DeleteExecutor {
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
    pub fn execute<E: EntityKind>(self, query: DeleteQuery) -> Result<DeleteResponse, Error> {
        let res = self.execute_internal::<E>(query)?;

        Ok(res)
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        &self,
        query: DeleteQuery,
    ) -> Result<DeleteResponse, DataError> {
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

        // debug
        self.debug
            .println(&format!("query.delete: delete keys {sort_keys:?}"));

        // get store
        let store = self
            .data
            .with(|db| db.try_get_store(resolved.store_path()))?;

        //
        // execute for every different key
        //
        let mut deleted_keys = Vec::new();

        for sk in sort_keys {
            if let Some(data_value) = store.with_borrow(|store| store.get(&sk)) {
                // Step 1: Deserialize the entity
                let data = &data_value.data;
                let entity: E = deserialize(data)?;
                let indexes = resolved.indexes();

                // Step 2: Extract field values
                let field_values = entity.key_values();

                // Step 3: Compute and delete index keys
                for index in indexes {
                    let index_key = resolved.build_index_key(index, &field_values);
                    let index_store = self.indexes.with(|ix| ix.try_get_store(&index.store))?;

                    self.debug
                        .println(&format!("query.delete: delete index {index_key:?}"));

                    index_store.with_borrow_mut(|store| {
                        store.data.remove(&index_key);
                    });
                }

                // Step 4: Delete the data row itself
                store.with_borrow_mut(|store| {
                    store.data.remove(&sk);
                });

                deleted_keys.push(sk);
            }
        }

        // debug
        self.debug
            .println(&format!("query.delete: deleted keys {deleted_keys:?}"));

        Ok(DeleteResponse(deleted_keys))
    }
}
