use crate::{
    MimicError,
    core::traits::EntityKind,
    db::{
        DbError,
        executor::FilterEvaluator,
        query::{FilterExpr, LoadFormat, LoadQuery, QueryPlan, QueryRange, QueryShape, Selector},
        response::{EntityRow, LoadCollection, LoadResponse},
        store::{
            DataKey, DataKeyRange, DataRow, DataStoreLocal, DataStoreRegistry, IndexStoreRegistry,
        },
    },
    debug,
};

///
/// LoadExecutor
///

#[derive(Clone, Copy, Debug)]
pub struct LoadExecutor {
    data_registry: DataStoreRegistry,
    index_registry: IndexStoreRegistry,
    debug: bool,
}

impl LoadExecutor {
    // new
    #[must_use]
    pub const fn new(data_registry: DataStoreRegistry, index_registry: IndexStoreRegistry) -> Self {
        Self {
            data_registry,
            index_registry,
            debug: false,
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    // execute
    pub fn execute<E: EntityKind>(self, query: LoadQuery) -> Result<LoadCollection<E>, MimicError> {
        let cl = self.execute_internal::<E>(query)?;

        Ok(cl)
    }

    // execute_response
    pub fn execute_response<E: EntityKind>(
        self,
        query: LoadQuery,
    ) -> Result<LoadResponse, MimicError> {
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
    ) -> Result<LoadCollection<E>, DbError> {
        debug!(self.debug, "query.load: {query:?}");

        // cast results to E
        let rows = self
            .load::<E>(&query.selector, query.filter.as_ref())?
            .into_iter()
            .filter(|row| row.entry.path == E::PATH)
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()?;

        // post filters
        let mut rows = Self::apply_filter(rows, &query);
        Self::apply_sort(&mut rows, &query);

        // paginate
        rows = rows
            .into_iter()
            .skip(query.offset as usize)
            .take(query.limit.unwrap_or(u32::MAX) as usize)
            .collect::<Vec<_>>();

        Ok(LoadCollection(rows))
    }

    // load
    pub fn load<E: EntityKind>(
        &self,
        selector: &Selector,
        filter: Option<&FilterExpr>,
    ) -> Result<Vec<DataRow>, DbError> {
        let plan = QueryPlan::new(selector.resolve::<E>(), filter.cloned());

        self.execute_plan::<E>(plan)
    }

    // execute_plan
    fn execute_plan<E: EntityKind>(&self, plan: QueryPlan) -> Result<Vec<DataRow>, DbError> {
        let store = self.data_registry.with(|db| db.try_get_store(E::STORE))?;

        let rows = match plan.shape {
            QueryShape::One(key) => Self::load_one(store, key).into_iter().collect(),

            QueryShape::Many(keys) => keys
                .into_iter()
                .filter_map(|key| Self::load_one(store, key))
                .collect(),

            QueryShape::Range(range) => Self::load_range_with_bounds::<E>(store, range),

            QueryShape::All => store.with_borrow(|this| {
                this.iter()
                    .map(|(key, entry)| DataRow { key, entry })
                    .collect()
            }),
        };

        Ok(rows)
    }

    // load_one
    fn load_one(store: DataStoreLocal, key: DataKey) -> Option<DataRow> {
        store.with_borrow(|this| {
            this.get(&key).map(|entry| DataRow {
                key: key.clone(),
                entry,
            })
        })
    }

    // load_range_with_bounds
    fn load_range_with_bounds<E: EntityKind>(
        store: DataStoreLocal,
        range: QueryRange,
    ) -> Vec<DataRow> {
        let data_range = range.to_data_key_range::<E>();

        store.with_borrow(|this| match data_range {
            DataKeyRange::Inclusive(r) => this.range(r).map(|(k, v)| DataRow::new(k, v)).collect(),
            DataKeyRange::Exclusive(r) => this.range(r).map(|(k, v)| DataRow::new(k, v)).collect(),

            DataKeyRange::SkipFirstInclusive(r) => this
                .range(r)
                .skip(1)
                .map(|(k, v)| DataRow::new(k, v))
                .collect(),

            DataKeyRange::SkipFirstExclusive(r) => this
                .range(r)
                .skip(1)
                .map(|(k, v)| DataRow::new(k, v))
                .collect(),
        })
    }

    // apply_filter
    fn apply_filter<E: EntityKind>(
        rows: Vec<EntityRow<E>>,
        query: &LoadQuery,
    ) -> Vec<EntityRow<E>> {
        match &query.filter {
            Some(expr_raw) => {
                let expr = expr_raw.clone().simplify(); // ⬅️ done once

                rows.into_iter()
                    .filter(|row| {
                        let values = row.entry.entity.values();
                        FilterEvaluator::new(&values).eval(&expr)
                    })
                    .collect()
            }
            None => rows,
        }
    }

    // apply_sort
    fn apply_sort<E: EntityKind>(rows: &mut [EntityRow<E>], query: &LoadQuery) {
        if !query.sort.is_empty() {
            let sorter = E::sort(&query.sort);
            rows.sort_by(|a, b| sorter(&a.entry.entity, &b.entry.entity));
        }
    }
}
