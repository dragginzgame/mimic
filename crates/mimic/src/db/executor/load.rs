use crate::{
    Error,
    core::{
        Value,
        traits::{EntityKind, Path},
    },
    db::{
        DbError,
        executor::FilterEvaluator,
        query::{
            FilterBuilder, FilterExpr, LoadQuery, QueryPlan, QueryPlanner, QueryValidate,
            SortDirection, SortExpr,
        },
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

    pub fn one<E: EntityKind>(&self, value: impl Into<Value>) -> Result<E, Error> {
        self.execute::<E>(LoadQuery::new().one::<E>(value))?
            .try_entity()
    }

    pub fn many<E, I>(&self, values: I) -> Result<LoadCollection<E>, Error>
    where
        E: EntityKind,
        I: IntoIterator,
        I::Item: Into<Value>,
    {
        self.execute::<E>(LoadQuery::new().many::<E, I>(values))
    }

    pub fn all<E: EntityKind>(&self) -> Result<LoadCollection<E>, Error> {
        self.execute::<E>(LoadQuery::new())
    }

    pub fn filter<E: EntityKind>(
        &self,
        f: impl FnOnce(FilterBuilder) -> FilterBuilder,
    ) -> Result<LoadCollection<E>, Error> {
        self.execute::<E>(LoadQuery::new().filter(f))
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn count_all<E: EntityKind>(self) -> Result<u32, Error> {
        self.count::<E>(LoadQuery::all())
    }

    //
    // EXECUTION LOGIC
    //

    /// Execute a full query and return a collection of entities.
    pub fn execute<E: EntityKind>(&self, query: LoadQuery) -> Result<LoadCollection<E>, Error> {
        QueryValidate::<E>::validate(&query).map_err(DbError::from)?;

        let plan = self.build_plan::<E>(query.filter.as_ref());
        let rows = self.execute_plan::<E>(&plan)?;
        let entities = Self::finalize_rows::<E>(rows, &query)?;

        Ok(LoadCollection(entities))
    }

    /// Executes a query and returns only the count
    /// matching entities using lazy iteration without full deserialization.
    #[allow(clippy::cast_possible_truncation)]
    pub fn count<E: EntityKind>(self, query: LoadQuery) -> Result<u32, Error> {
        QueryValidate::<E>::validate(&query).map_err(DbError::from)?;

        let plan = self.build_plan::<E>(query.filter.as_ref());

        match &plan {
            QueryPlan::Index(index_plan) => {
                let index_store = self
                    .index_registry
                    .with(|reg| reg.try_get_store(index_plan.index.store))
                    .map_err(DbError::from)?;

                let keys = index_store.with_borrow(|istore| {
                    istore.resolve_data_keys::<E>(index_plan.index, &index_plan.keys)
                });

                return Ok(keys.len() as u32);
            }

            QueryPlan::Keys(keys) => {
                return Ok(keys.len() as u32);
            }

            _ => {}
        }

        // Fallback: range scan with optional filter
        let rows = self.execute_plan::<E>(&plan)?;

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

    /// Build the plan
    fn build_plan<E: EntityKind>(&self, filter: Option<&FilterExpr>) -> QueryPlan {
        let planner = QueryPlanner::new(filter);
        let plan = planner.plan::<E>();

        debug!(self.debug, "query.plan: {plan:?}");

        plan
    }

    /// Execute only the raw data plan (no filters/sort/pagination yet)
    fn execute_plan<E: EntityKind>(&self, plan: &QueryPlan) -> Result<Vec<DataRow>, Error> {
        let store = self
            .data_registry
            .with(|reg| reg.try_get_store(E::Store::PATH))
            .map_err(DbError::from)?;

        let shape = match plan {
            QueryPlan::Keys(keys) => Self::load_many(store, keys),
            QueryPlan::Range(start, end) => Self::load_range(store, start.clone(), end.clone()),

            QueryPlan::Index(index_plan) => {
                let index_store = self
                    .index_registry
                    .with(|reg| reg.try_get_store(index_plan.index.store))
                    .map_err(DbError::from)?;

                let keys = index_store.with_borrow(|istore| {
                    istore.resolve_data_keys::<E>(index_plan.index, &index_plan.keys)
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
        let mut entities: Vec<_> = rows
            .into_iter()
            .map(EntityRow::<E>::try_from)
            .collect::<Result<_, _>>()?;

        // In-place filter
        if let Some(filter) = &query.filter {
            let filter_simple = filter.clone().simplify();

            entities.retain(|row| FilterEvaluator::new(&row.entry.entity).eval(&filter_simple));
        }

        // In-place sort
        if let Some(sort) = &query.sort {
            Self::apply_sort(&mut entities, sort);
        }

        // In-place pagination
        if let Some(limit) = &query.limit {
            let total = entities.len();
            let start = usize::min(limit.offset as usize, total);
            let end = match limit.limit {
                Some(lim) => usize::min(start + lim as usize, total),
                None => total,
            };

            // No heap reallocation — slicing the original buffer
            entities = entities[start..end].to_vec();
        }

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
        rows.into_iter()
            .filter(|row| FilterEvaluator::new(&row.entry.entity).eval(filter))
            .collect()
    }

    // apply_sort
    fn apply_sort<E: EntityKind>(rows: &mut [EntityRow<E>], sort_expr: &SortExpr) {
        rows.sort_by(|a, b| {
            for (field, direction) in sort_expr.iter() {
                let (Some(va), Some(vb)) = (
                    a.entry.entity.get_value(field),
                    b.entry.entity.get_value(field),
                ) else {
                    continue;
                };

                if let Some(ordering) = va.partial_cmp(&vb) {
                    return match direction {
                        SortDirection::Asc => ordering,
                        SortDirection::Desc => ordering.reverse(),
                    };
                }
            }

            core::cmp::Ordering::Equal
        });
    }
}
