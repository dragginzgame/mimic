use crate::{
    Error,
    db::{DataStoreRegistry, IndexStoreRegistry},
    query::{LoadCollectionDyn, LoadQueryDyn, LoadResponse},
    service::{
        ServiceError,
        storage::{DebugContext, Loader, StorageError, with_resolver},
    },
    traits::Entity,
};

///
/// LoadExecutorDyn
///

pub struct LoadExecutorDyn {
    data: DataStoreRegistry,
    indexes: IndexStoreRegistry,
    debug: DebugContext,
}

impl LoadExecutorDyn {
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
    pub fn execute<E: Entity>(self, query: LoadQueryDyn) -> Result<LoadCollectionDyn, Error> {
        let res = self
            .execute_internal::<E>(query)
            .map_err(ServiceError::from)?;

        Ok(res)
    }

    // response
    pub fn response<E: Entity>(self, query: LoadQueryDyn) -> Result<LoadResponse, Error> {
        let format = query.format;
        let cll = self
            .execute_internal::<E>(query)
            .map_err(ServiceError::from)?;

        Ok(cll.response(format))
    }

    // execute_internal
    fn execute_internal<E: Entity>(
        self,
        query: LoadQueryDyn,
    ) -> Result<LoadCollectionDyn, StorageError> {
        self.debug.println(&format!("query.load_dyn: {query:?}"));

        // store
        let store_path = with_resolver(|r| r.resolve_store(E::PATH))?;
        let store = self.data.with(|db| db.try_get_store(&store_path))?;

        // selector
        let selector = with_resolver(|r| r.resolve_selector(E::PATH, &query.selector))?;

        // loader
        let loader = Loader::new(store);
        let res = loader.load(&selector);

        // paginate and filter incorrect paths
        let rows = res
            .into_iter()
            .filter(|row| {
                query.include_children && row.value.path.starts_with(E::PATH)
                    || row.value.path == E::PATH
            })
            .skip(query.offset as usize)
            .take(query.limit.unwrap_or(u32::MAX) as usize)
            .collect();

        Ok(LoadCollectionDyn(rows))
    }
}
