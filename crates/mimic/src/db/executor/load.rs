use crate::{
    MimicError,
    core::{Value, traits::EntityKind},
    db::{
        DbError,
        executor::FilterEvaluator,
        query::{FilterExpr, LoadQuery, QueryPlanner, QueryShape, SortExpr},
        response::{EntityRow, LoadCollection},
        store::{
            DataKey, DataRow, DataStoreLocal, DataStoreRegistryLocal, IndexStoreRegistryLocal,
        },
    },
    debug,
};

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
        let rows = self.execute_plan::<E>(query.filter.as_ref());

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
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn count_all<E: EntityKind>(self) -> u32 {
        let rows = self.execute_plan::<E>(None);

        rows.len() as u32
    }

    /// Internal query executor: handles plan → data rows → filtering/sorting/pagination
    fn execute_internal<E: EntityKind>(
        &self,
        query: LoadQuery,
    ) -> Result<LoadCollection<E>, DbError> {
        let rows = self.execute_plan::<E>(query.filter.as_ref());
        let entities = Self::finalize_rows::<E>(rows, &query)?;

        Ok(LoadCollection(entities))
    }

    /// Execute only the raw data plan (no filters/sort/pagination yet)
    fn execute_plan<E: EntityKind>(&self, filter: Option<&FilterExpr>) -> Vec<DataRow> {
        // create planner
        let planner = QueryPlanner::new(filter);
        let plan = planner.plan_with_registry::<E>(self.index_registry);

        debug!(self.debug, "query.load: plan: {plan:?}");

        let store = self.data_registry.with(|reg| reg.get_store::<E::Store>());

        match plan.shape {
            QueryShape::All => store.with_borrow(|this| {
                this.iter_pairs()
                    .map(|(key, entry)| DataRow { key, entry })
                    .collect()
            }),

            QueryShape::One(key) => Self::load_one(store, &key).into_iter().collect(),

            QueryShape::Many(keys) => keys
                .into_iter()
                .filter_map(|key| Self::load_one(store, &key))
                .collect(),

            QueryShape::Range(start, end) => Self::load_range(store, &start, &end),
        }
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

    // load_one
    fn load_one(store: DataStoreLocal, key: &DataKey) -> Option<DataRow> {
        store.with_borrow(|this| {
            this.get(key).map(|entry| DataRow {
                key: key.clone(),
                entry,
            })
        })
    }

    // load_range
    fn load_range(store: DataStoreLocal, start: &DataKey, end: &DataKey) -> Vec<DataRow> {
        store.with_borrow(|this| {
            this.range_pairs(start..=end)
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

    // apply_pagination
    fn apply_pagination<T>(rows: Vec<T>, offset: u32, limit: Option<u32>) -> Vec<T> {
        rows.into_iter()
            .skip(offset as usize)
            .take(limit.unwrap_or(u32::MAX) as usize)
            .collect()
    }
}
