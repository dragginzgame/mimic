use crate::{
    Error,
    db::{
        DataError,
        executor::Loader,
        query::{LoadFormat, LoadQuery},
        response::{LoadCollection, LoadResponse},
        store::{DataStoreRegistry, IndexStoreRegistry},
        types::EntityRow,
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
            LoadFormat::Rows => LoadResponse::Rows(cl.data_rows()),
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

        let loader = Loader::new(self.data_registry, self.index_registry, self.debug);

        let rows = loader
            .load::<E>(&query.selector, query.r#where.as_ref())?
            .into_iter()
            .filter(|row| row.value.path == E::PATH)
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
                let field_values = row.value.entity.values();

                r#where.matches.iter().all(|(field, value)| {
                    field_values.get(field).and_then(|v| v.as_ref()) == Some(value)
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
            .filter(|row| row.value.entity.search_fields(&query.search))
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
            rows.sort_by(|a, b| sorter(&a.value.entity, &b.value.entity));
        }
    }
}
