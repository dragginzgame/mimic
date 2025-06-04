use crate::{
    Error,
    db::{
        DataStoreRegistry, IndexStoreRegistry,
        types::{DataValue, SortKey},
    },
    query::{DeleteQuery, DeleteResponse},
    service::{
        ServiceError,
        storage::{DebugContext, ResolvedSelector, StorageError, with_resolver},
    },
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
        let res = self
            .execute_internal::<E>(query)
            .map_err(ServiceError::from)?;

        Ok(res)
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        &self,
        query: DeleteQuery,
    ) -> Result<DeleteResponse, StorageError> {
        self.debug.println(&format!("query.delete: {query:?}"));

        // resolver
        let resolved = with_resolver(|r| r.entity(E::PATH))?;
        let indexes = resolved.indexes();

        // selector
        let selector = resolved
            .selector(&query.selector)
            .map_err(StorageError::from)?;

        let sort_keys: Vec<SortKey> = match selector {
            ResolvedSelector::One(key) => vec![key],
            ResolvedSelector::Many(keys) => keys,
            ResolvedSelector::Range(..) => return Err(StorageError::SelectorNotSupported),
        };

        // debug
        self.debug.println(&format!("delete: keys {sort_keys:?}"));

        // get store
        let store = self
            .data
            .with(|db| db.try_get_store(resolved.store_path()))
            .map_err(StorageError::DbError)?;

        //
        // execute for every different key
        //
        let mut deleted_keys = Vec::new();

        for sk in sort_keys {
            let maybe_value: Option<DataValue> =
                store.with_borrow(|store| store.get(&sk)).map(|v| v.clone());

            if let Some(_data_value) = maybe_value {
                /*
                                let e: Option<E> = data_value.try_into();

                                if let Ok(entity) = <E as TryFrom<DataValue>>::try_from(data_value) {
                                    // Step 2: extract field values from the row
                                    let field_values = entity.key_values();

                                    // Step 3: compute and delete index keys
                                    for index_key in resolved.index_keys_from_values(&field_values) {
                                        let index_store = self
                                            .indexes
                                            .with(|ix| ix.try_get_store(&index_key.entity))
                                            .map_err(StorageError::DbError)?;

                                        self.debug.println(&format!("index delete: {index_key:?}"));

                                        index_store.with_borrow_mut(|store| {
                                            store.remove(&index_key);
                                        });
                                    }
                */
                // Step 4: delete the row
                store.with_borrow_mut(|store| {
                    store.remove(&sk);
                });

                deleted_keys.push(sk);
            }
        }

        // debug
        self.debug
            .println(&format!("keys deleted: {deleted_keys:?}"));

        Ok(DeleteResponse(deleted_keys))
    }
}
