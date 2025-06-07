use crate::{
    Error,
    data::{
        DataError,
        executor::{DebugContext, Loader, ResolvedEntity, types::EntityRow, with_resolver},
        query::{LoadQuery, Where},
        response::{LoadCollection, LoadResponse},
        store::{DataStoreRegistry, IndexStoreRegistry, IndexValue},
    },
    traits::EntityKind,
};
use std::collections::HashMap;

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
    pub fn execute<E: EntityKind>(self, query: LoadQuery) -> Result<LoadCollection<E>, Error> {
        let cll = self.execute_internal(query)?;

        Ok(cll)
    }

    // response
    pub fn response<E: EntityKind>(self, query: LoadQuery) -> Result<LoadResponse, Error> {
        let format = query.format;
        let cll = self.execute_internal::<E>(query)?;

        Ok(cll.response(format))
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        self,
        query: LoadQuery,
    ) -> Result<LoadCollection<E>, DataError> {
        // resolver
        self.debug.println(&format!("query.load: {query:?}"));
        let resolved = with_resolver(|r| r.entity(E::PATH))?;
        let store = self
            .data
            .with(|db| db.try_get_store(resolved.store_path()))?;

        // selector
        let selector = resolved.selector(&query.selector);
        self.debug
            .println(&format!("query.load selector: {selector:?}"));

        // loader
        let res = Loader::new(store, self.debug).load(&selector);
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

    // try_index_lookup
    fn try_index_lookup(
        &self,
        resolved: &ResolvedEntity,
        where_clause: &Where,
    ) -> Result<Option<IndexValue>, DataError> {
        // Build a map from field → Some(value) directly (for build_index_key)
        let field_values: HashMap<_, _> = where_clause
            .matches
            .iter()
            .map(|(k, v)| (k.clone(), Some(v.clone())))
            .collect();

        for index in resolved.indexes() {
            // Ensure all index fields are present in the where clause
            if index.fields.iter().all(|f| field_values.contains_key(f)) {
                // Try to build the index key from ordered fields and optional values
                let Some(index_key) = resolved.build_index_key(index, &field_values) else {
                    self.debug.println(&format!(
                        "query.load: skipping index {:?} due to null/empty value",
                        index.fields
                    ));
                    continue;
                };

                let store = self.indexes.with(|map| map.try_get_store(&index.store))?;

                return Ok(store.with_borrow(|s| s.get(&index_key)));
            }
        }

        Ok(None)
    }
}

// apply_filters
fn apply_filters<E: EntityKind>(rows: Vec<EntityRow<E>>, query: &LoadQuery) -> Vec<EntityRow<E>> {
    let use_search = !query.search.is_empty();

    rows.into_iter()
        .filter(|row| {
            let entity = &row.value.entity;
            let key_values = entity.key_values();

            let where_ok = query.r#where.as_ref().is_none_or(|w| {
                w.matches.iter().all(|(field, value)| {
                    key_values.get(field).and_then(|v| v.as_ref()) == Some(value)
                })
            });

            let search_ok = !use_search || entity.search_fields(&query.search);

            where_ok && search_ok
        })
        .collect()
}

// apply_sort
fn apply_sort<E: EntityKind>(mut rows: Vec<EntityRow<E>>, query: &LoadQuery) -> Vec<EntityRow<E>> {
    if !query.sort.is_empty() {
        let sorter = E::sort(&query.sort);
        rows.sort_by(|a, b| sorter(&a.value.entity, &b.value.entity));
    }

    rows
}

// apply_pagination
fn apply_pagination<E: EntityKind>(
    rows: Vec<EntityRow<E>>,
    query: &LoadQuery,
) -> Vec<EntityRow<E>> {
    let (offset, limit) = (query.offset, query.limit.unwrap_or(u32::MAX));

    rows.into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect()
}
