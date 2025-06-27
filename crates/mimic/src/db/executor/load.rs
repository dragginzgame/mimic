use crate::{
    Error,
    db::{
        DataError,
        executor::ResolvedSelector,
        query::{LoadFormat, LoadQuery, Selector, Where},
        response::{EntityRow, LoadCollection, LoadResponse},
        store::{DataKey, DataRow, DataStoreLocal, DataStoreRegistry, IndexStoreRegistry},
    },
    debug,
    ops::traits::EntityKind,
};
use icu::{Log, log};

///
/// LoadExecutor
///

#[allow(clippy::type_complexity)]
pub struct LoadExecutor {
    data_registry: DataStoreRegistry,
    index_registry: IndexStoreRegistry,
    debug: bool,
}

impl LoadExecutor {
    // new
    #[must_use]
    pub fn new(data_registry: DataStoreRegistry, index_registry: IndexStoreRegistry) -> Self {
        Self {
            data_registry,
            index_registry,
            debug: false,
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug = false;
        self
    }

    // execute
    pub fn execute<E: EntityKind>(self, query: LoadQuery) -> Result<LoadCollection<E>, Error> {
        let cl = self.execute_internal::<E>(query)?;

        Ok(cl)
    }

    // execute_response
    pub fn execute_response<E: EntityKind>(self, query: LoadQuery) -> Result<LoadResponse, Error> {
        let format = query.format;
        let cl = self.execute_internal::<E>(query)?;

        let resp = match format {
            LoadFormat::Keys => LoadResponse::Keys(cl.keys()),
            LoadFormat::Count => LoadResponse::Count(cl.count()),
        };

        Ok(resp)
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        self,
        query: LoadQuery,
    ) -> Result<LoadCollection<E>, DataError> {
        debug!(self.debug, "query.load: {query:?}");

        let rows = self
            .load::<E>(&query.selector, query.r#where.as_ref())?
            .into_iter()
            .filter(|row| row.entry.path == E::PATH)
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()?;

        // apply post filters and paginate
        let rows = Self::apply_all_post(rows, &query)
            .into_iter()
            .skip(query.offset as usize)
            .take(query.limit.unwrap_or(u32::MAX) as usize)
            .collect();

        Ok(LoadCollection(rows))
    }

    // load
    pub fn load<E: EntityKind>(
        &self,
        selector: &Selector,
        _where_clause: Option<&Where>,
    ) -> Result<Vec<DataRow>, DataError> {
        // TODO - big where_clause changing selector thingy
        // get store
        let store = self.data_registry.with(|db| db.try_get_store(E::STORE))?;
        let resolved_selector = selector.resolve::<E>();

        // load rows
        let rows = match resolved_selector {
            ResolvedSelector::One(key) => self.load_one(store, key).into_iter().collect(),

            ResolvedSelector::Many(keys) => keys
                .into_iter()
                .filter_map(|key| self.load_one(store, key))
                .collect(),

            ResolvedSelector::Range(start, end) => self.load_range(store, start, end),
        };

        Ok(rows)
    }

    // load_one
    fn load_one(&self, store: DataStoreLocal, key: DataKey) -> Option<DataRow> {
        store.with_borrow(|this| {
            this.get(&key).map(|entry| DataRow {
                key: key.clone(),
                entry,
            })
        })
    }

    // load_range
    fn load_range(&self, store: DataStoreLocal, start: DataKey, end: DataKey) -> Vec<DataRow> {
        store.with_borrow(|this| {
            this.range(start..=end)
                .map(|(key, entry)| DataRow { key, entry })
                .collect()
        })
    }

    // apply_all_post
    // noisy but more efficient, so keeping it in its own method
    fn apply_all_post<E: EntityKind>(
        rows: Vec<EntityRow<E>>,
        query: &LoadQuery,
    ) -> Vec<EntityRow<E>> {
        let rows = Self::apply_where(rows, query);
        let mut rows = Self::apply_search(rows, query);
        Self::apply_sort(&mut rows, query);

        rows
    }

    // apply_where
    fn apply_where<E: EntityKind>(rows: Vec<EntityRow<E>>, query: &LoadQuery) -> Vec<EntityRow<E>> {
        let Some(r#where) = query.r#where.as_ref() else {
            return rows;
        };
        let olen = rows.len();

        // filter
        let filtered = rows
            .into_iter()
            .filter(|row| {
                let values = row.entry.entity.values();

                r#where
                    .matches
                    .iter()
                    .all(|(field, expected)| match values.get(field) {
                        Some(Some(actual)) => actual == expected,
                        _ => false,
                    })
            })
            .collect::<Vec<_>>();
        let flen = filtered.len();

        if flen < olen {
            log!(Log::Info, "apply_where: filtered {olen} → {flen} rows",);
        }

        filtered
    }

    // apply_search
    fn apply_search<E: EntityKind>(
        rows: Vec<EntityRow<E>>,
        query: &LoadQuery,
    ) -> Vec<EntityRow<E>> {
        if query.search.is_empty() {
            return rows;
        }
        let olen = rows.len();

        // filter
        let filtered = rows
            .into_iter()
            .filter(|row| row.entry.entity.search_fields(&query.search))
            .collect::<Vec<_>>();
        let flen = filtered.len();

        if flen < olen {
            log!(Log::Info, "apply_search: filtered {olen} → {flen} rows",);
        }

        filtered
    }

    // apply_sort
    fn apply_sort<E: EntityKind>(rows: &mut [EntityRow<E>], query: &LoadQuery) {
        if !query.sort.is_empty() {
            let sorter = E::sort(&query.sort);
            rows.sort_by(|a, b| sorter(&a.entry.entity, &b.entry.entity));
        }
    }
}
