use crate::{
    MimicError,
    core::{
        Value,
        traits::{EntityKind, Path},
    },
    db::{
        DbError,
        executor::FilterEvaluator,
        query::{FilterExpr, LoadQuery, QueryPlan, QueryPlanner, SortExpr},
        response::{EntityRow, LoadCollection},
        store::{
            DataKey, DataRow, DataStoreLocal, DataStoreRegistryLocal, IndexStoreRegistryLocal,
        },
    },
    debug,
};
use std::ops::Bound;

///
/// LoadExecutor
///

#[derive(Clone, Copy, Debug)]
pub struct LoadExecutor {
    data_registry: DataStoreRegistryLocal,
    index_registry: IndexStoreRegistryLocal,
    debug: bool,
}

impl LoadExecutor {
    // new
    #[must_use]
    pub const fn new(
        data_registry: DataStoreRegistryLocal,
        index_registry: IndexStoreRegistryLocal,
    ) -> Self {
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

    //
    // HELPER METHODS
    // these will create a query on the fly
    //

    // one
    pub fn one<E: EntityKind>(&self, value: impl Into<Value>) -> Result<E, MimicError> {
        self.execute::<E>(LoadQuery::new().one::<E>(value))?
            .try_entity()
    }

    // many
    pub fn many<E, I>(&self, values: I) -> Result<LoadCollection<E>, MimicError>
    where
        E: EntityKind,
        I: IntoIterator,
        I::Item: Into<Value>,
    {
        self.execute::<E>(LoadQuery::new().many::<E, I>(values))
    }

    // all
    pub fn all<E: EntityKind>(&self) -> Result<LoadCollection<E>, MimicError> {
        self.execute::<E>(LoadQuery::new())
    }

    // filter
    pub fn filter<E: EntityKind>(&self) -> Result<LoadCollection<E>, MimicError> {
        self.execute::<E>(LoadQuery::new())
    }

    //
    // EXECUTION LOGIC
    //

    /// Execute a full query and return a collection of entities.
    pub fn execute<E: EntityKind>(self, query: LoadQuery) -> Result<LoadCollection<E>, MimicError> {
        let collection = self.execute_internal::<E>(query)?;

        Ok(collection)
    }

    /// Count matching entities using lazy iteration without full deserialization.
    #[allow(clippy::cast_possible_truncation)]
    pub fn count<E: EntityKind>(self, query: LoadQuery) -> Result<u32, MimicError> {
        // Only takes filter into account
        let rows = self.execute_plan::<E>(query.filter.as_ref())?;

        // filter or not?
        let count = if let Some(filter) = &query.filter {
            let filtered = Self::apply_filter::<E>(
                rows.into_iter()
                    .map(TryFrom::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
                filter,
            );

            filtered.len() as u32
        } else {
            rows.len() as u32
        };

        Ok(count)
    }

    /// count_all
    #[allow(clippy::cast_possible_truncation)]
    pub fn count_all<E: EntityKind>(self) -> Result<u32, DbError> {
        let rows = self.execute_plan::<E>(None)?;

        Ok(rows.len() as u32)
    }

    /// Internal query executor: handles plan → data rows → filtering/sorting/pagination
    fn execute_internal<E: EntityKind>(
        &self,
        query: LoadQuery,
    ) -> Result<LoadCollection<E>, DbError> {
        let rows = self.execute_plan::<E>(query.filter.as_ref())?;
        let entities = Self::finalize_rows::<E>(rows, &query)?;

        Ok(LoadCollection(entities))
    }

    /// Execute only the raw data plan (no filters/sort/pagination yet)
    fn execute_plan<E: EntityKind>(
        &self,
        filter: Option<&FilterExpr>,
    ) -> Result<Vec<DataRow>, DbError> {
        // create planner
        let planner = QueryPlanner::new(filter);
        let plan = planner.plan::<E>();

        debug!(self.debug, "query.load: plan: {plan:?}");

        // get store
        let store = self
            .data_registry
            .with(|reg| reg.try_get_store(E::Store::PATH))?;

        let shape = match plan {
            QueryPlan::Keys(keys) => Self::load_many(store, &keys),
            QueryPlan::Range(start, end) => Self::load_range(store, start, end),

            QueryPlan::Index(plan) => {
                // get the index store
                let index_store = self
                    .index_registry
                    .with(|reg| reg.try_get_store(plan.store_path))?;

                // resolve keys
                let keys = index_store.with_borrow(|istore| {
                    istore.resolve_data_keys::<E>(plan.index_path, plan.index_fields, &plan.keys)
                });

                Self::load_many(store, &keys)
            }
        };

        Ok(shape)
    }

    // finalize_rows
    fn finalize_rows<E: EntityKind>(
        rows: Vec<DataRow>,
        query: &LoadQuery,
    ) -> Result<Vec<EntityRow<E>>, DbError> {
        let mut entities = rows
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()?;

        // filter
        if let Some(filter) = &query.filter {
            entities = Self::apply_filter(entities, filter);
        }

        // sort
        if let Some(sort) = &query.sort {
            Self::apply_sort(&mut entities, sort);
        }

        // paginate
        entities = Self::apply_pagination(entities, query.offset, query.limit);

        Ok(entities)
    }

    // load_many
    fn load_many(store: DataStoreLocal, keys: &[DataKey]) -> Vec<DataRow> {
        store.with_borrow(|this| {
            keys.iter()
                .filter_map(|key| {
                    this.get(key).map(|entry| DataRow {
                        key: key.clone(),
                        entry,
                    })
                })
                .collect()
        })
    }

    // load_range
    fn load_range(store: DataStoreLocal, start: DataKey, end: DataKey) -> Vec<DataRow> {
        store.with_borrow(|this| {
            this.range((Bound::Included(start), Bound::Included(end)))
                .map(|entry| DataRow::new(entry.key().clone(), entry.value()))
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

    // apply_pagination
    fn apply_pagination<T>(rows: Vec<T>, offset: u32, limit: Option<u32>) -> Vec<T> {
        rows.into_iter()
            .skip(offset as usize)
            .take(limit.unwrap_or(u32::MAX) as usize)
            .collect()
    }
}
