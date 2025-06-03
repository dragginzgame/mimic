use crate::{
    Error,
    db::{DataStoreRegistry, IndexStoreRegistry, types::SortKey},
    query::{DeleteQuery, DeleteResponse},
    service::{
        ServiceError,
        storage::{DebugContext, ResolvedSelector, StorageError, with_resolver},
    },
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
    pub fn execute(self, query: DeleteQuery) -> Result<DeleteResponse, Error> {
        let res = self.execute_internal(query).map_err(ServiceError::from)?;

        Ok(res)
    }

    // execute_internal
    fn execute_internal(&self, query: DeleteQuery) -> Result<DeleteResponse, StorageError> {
        // resolved_entity
        let resolved_entity = with_resolver(|r| r.entity(&query.path))?;

        // selector
        let selector = resolved_entity
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
            .with(|db| db.try_get_store(resolved_entity.store_path()))
            .map_err(StorageError::DbError)?;

        // execute for every different key
        let mut deleted_keys = Vec::new();
        for sk in sort_keys {
            // remove returns DataValue but we ignore it for now
            // if the key is deleted then add it to the vec
            if store.with_borrow_mut(|store| store.remove(&sk)).is_some() {
                deleted_keys.push(sk);
            }
        }

        // debug
        self.debug
            .println(&format!("keys deleted: {deleted_keys:?}"));

        Ok(DeleteResponse(deleted_keys))
    }
}
