use crate::{
    MimicError,
    core::traits::EntityKind,
    db::{
        DbError,
        executor::FilterEvaluator,
        query::{FilterExpr, LoadFormat, LoadQuery, QueryPlan, QueryShape, RangeExpr, SortExpr},
        response::{EntityRow, LoadCollection, LoadResponse},
        store::{DataKey, DataRow, DataStoreLocal, DataStoreRegistry, IndexStoreRegistry},
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
        let plan = QueryPlan::new(&query.range, &query.filter);

        // cast results to E
        let mut rows = self
            .execute_plan::<E>(&plan)?
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()?;

        // filter
        if let Some(filter) = &query.filter {
            rows = Self::apply_filter(rows, filter);
        }

        // sort
        if let Some(sort) = &query.sort {
            Self::apply_sort(&mut rows, sort);
        }

        // paginate
        rows = rows
            .into_iter()
            .skip(query.offset as usize)
            .take(query.limit.unwrap_or(u32::MAX) as usize)
            .collect::<Vec<_>>();

        Ok(LoadCollection(rows))
    }

    // execute_plan
    fn execute_plan<E: EntityKind>(&self, plan: &QueryPlan) -> Result<Vec<DataRow>, DbError> {
        let store = self.data_registry.with(|db| db.try_get_store(E::STORE))?;

        let shape = plan.shape::<E>();
        debug!(self.debug, "query.load: {plan:?} shape is {shape:?}");

        let rows = match shape {
            QueryShape::All => store.with_borrow(|this| {
                this.iter()
                    .map(|(key, entry)| DataRow { key, entry })
                    .collect()
            }),

            QueryShape::One(key) => Self::load_one(store, key).into_iter().collect(),

            QueryShape::Many(keys) => keys
                .into_iter()
                .filter_map(|key| Self::load_one(store, key))
                .collect(),

            QueryShape::Range(range) => Self::load_range(store, range),
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

    // load_range
    fn load_range(store: DataStoreLocal, range: RangeExpr) -> Vec<DataRow> {
        store.with_borrow(|this| {
            this.range(range.start..range.end)
                .map(|(key, entry)| DataRow { key, entry })
                .collect()
        })
    }

    // apply_filter
    fn apply_filter<E: EntityKind>(
        rows: Vec<EntityRow<E>>,
        filter: &FilterExpr,
    ) -> Vec<EntityRow<E>> {
        let filter_simple = filter.clone().simplify(); // ⬅️ done once

        rows.into_iter()
            .filter(|row| {
                let values = row.entry.entity.values();
                FilterEvaluator::new(&values).eval(&filter_simple)
            })
            .collect()
    }

    // apply_sort
    fn apply_sort<E: EntityKind>(rows: &mut [EntityRow<E>], sort: &SortExpr) {
        let sorter = E::sort(sort);
        rows.sort_by(|a, b| sorter(&a.entry.entity, &b.entry.entity));
    }
}
