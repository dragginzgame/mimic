use crate::{
    Error,
    db::{DataStoreRegistry, IndexStoreRegistry, types::EntityRow},
    query::{LoadCollection, LoadQueryInternal, LoadResponse},
    service::{
        ServiceError,
        storage::{DebugContext, Loader, StorageError, with_resolver},
    },
    traits::EntityKind,
};

///
/// LoadExecutor
///

#[allow(clippy::type_complexity)]
pub struct LoadExecutor {
    data: DataStoreRegistry,
    indexes: IndexStoreRegistry,
    debug: DebugContext,
}

impl LoadExecutor {
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
        self,
        query: LoadQueryInternal<E>,
    ) -> Result<LoadCollection<E>, Error> {
        let cll = self.execute_internal(query).map_err(ServiceError::from)?;

        Ok(cll)
    }

    // response
    pub fn response<E: EntityKind>(
        self,
        query: LoadQueryInternal<E>,
    ) -> Result<LoadResponse, Error> {
        let format = query.inner.format;
        let cll = self.execute_internal(query).map_err(ServiceError::from)?;

        Ok(cll.response(format))
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        self,
        query: LoadQueryInternal<E>,
    ) -> Result<LoadCollection<E>, StorageError> {
        // debug
        self.debug.println(&format!("query.load: {query:?}"));

        // resolver
        let resolved = with_resolver(|r| r.entity(E::PATH))?;
        let store = self
            .data
            .with(|db| db.try_get_store(resolved.store_path()))?;

        // resolved_selector
        let resolved_selector = resolved.selector(&query.inner.selector)?;
        self.debug.println(&format!(
            "query.load resolved_selector: {resolved_selector:?}"
        ));

        // loader
        let res = Loader::new(store).load(&resolved_selector);
        let rows = res
            .into_iter()
            .filter(|row| row.value.path == E::PATH)
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()?;

        // do stuff
        let rows = apply_filters(rows, &query);
        let rows = apply_sort(rows, &query);
        let rows = apply_pagination(rows, &query);

        Ok(LoadCollection(rows))
    }
}

// apply_filters
fn apply_filters<E: EntityKind>(
    rows: Vec<EntityRow<E>>,
    query: &LoadQueryInternal<E>,
) -> Vec<EntityRow<E>> {
    rows.into_iter()
        .filter(|row| {
            let entity = &row.value.entity;

            let matches_search =
                query.inner.search.is_empty() || entity.search_fields(&query.inner.search);
            let matches_custom_filters = query.filters.iter().all(|f| f(entity));

            matches_search && matches_custom_filters
        })
        .collect()
}

// apply_sort
fn apply_sort<E: EntityKind>(
    mut rows: Vec<EntityRow<E>>,
    query: &LoadQueryInternal<E>,
) -> Vec<EntityRow<E>> {
    if !query.inner.sort.is_empty() {
        let sorter = E::sort(&query.inner.sort);
        rows.sort_by(|a, b| sorter(&a.value.entity, &b.value.entity));
    }
    rows
}

// apply_pagination
fn apply_pagination<E: EntityKind>(
    rows: Vec<EntityRow<E>>,
    query: &LoadQueryInternal<E>,
) -> Vec<EntityRow<E>> {
    let (offset, limit) = (query.inner.offset, query.inner.limit.unwrap_or(u32::MAX));

    rows.into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect()
}
