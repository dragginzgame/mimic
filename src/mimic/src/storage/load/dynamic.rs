use crate::{
    Error,
    db::{DataStoreRegistry, IndexStoreRegistry},
    query::{LoadCollectionDyn, LoadQueryDyn, LoadResponse},
    storage::{DebugContext, Loader, StorageError, with_resolver},
    traits::EntityKind,
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
    pub fn execute<E: EntityKind>(self, query: LoadQueryDyn) -> Result<LoadCollectionDyn, Error> {
        let res = self.execute_internal::<E>(query)?;

        Ok(res)
    }

    // response
    pub fn response<E: EntityKind>(self, query: LoadQueryDyn) -> Result<LoadResponse, Error> {
        let format = query.format;
        let cll = self.execute_internal::<E>(query)?;

        Ok(cll.response(format))
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        self,
        query: LoadQueryDyn,
    ) -> Result<LoadCollectionDyn, StorageError> {
        self.debug.println(&format!("query.load_dyn: {query:?}"));

        // resolver
        let resolved = with_resolver(|r| r.entity(E::PATH))?;
        let store = self
            .data
            .with(|db| db.try_get_store(resolved.store_path()))?;
        let selector = resolved.selector(&query.selector);

        // loader
        let loader = Loader::new(store, self.debug);
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
