use crate::{
    Error,
    core::{
        Key, Value, deserialize,
        traits::{EntityKind, Path},
    },
    db::{
        DbError,
        executor::FilterEvaluator,
        query::{DeleteQuery, FilterBuilder, QueryPlan, QueryPlanner, QueryValidate},
        store::{DataKey, DataStoreRegistryLocal, IndexStoreRegistryLocal},
    },
    debug,
};
use std::ops::Bound;

///
/// DeleteExecutor
///

#[derive(Clone, Copy, Debug)]
pub struct DeleteExecutor {
    data_registry: DataStoreRegistryLocal,
    index_registry: IndexStoreRegistryLocal,
    debug: bool,
}

impl DeleteExecutor {
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

    ///
    /// HELPER METHODS
    /// these will create an intermediate query
    ///

    pub fn one<E: EntityKind>(&self, value: impl Into<Value>) -> Result<Vec<Key>, Error> {
        self.execute::<E>(DeleteQuery::new().one::<E>(value))
    }

    pub fn many<E: EntityKind>(
        &self,
        values: impl IntoIterator<Item = impl Into<Value>>,
    ) -> Result<Vec<Key>, Error> {
        self.execute::<E>(DeleteQuery::new().many::<E>(values))
    }

    pub fn all<E: EntityKind>(&self) -> Result<Vec<Key>, Error> {
        self.execute::<E>(DeleteQuery::new())
    }

    pub fn filter<E: EntityKind>(
        self,
        f: impl FnOnce(FilterBuilder) -> FilterBuilder,
    ) -> Result<Vec<Key>, Error> {
        self.execute::<E>(DeleteQuery::new().filter(f))
    }

    ///
    /// EXECUTION METHODS
    ///

    // response
    // for the automated query endpoint, we will make this more flexible in the future
    pub fn response<E: EntityKind>(self, query: DeleteQuery) -> Result<Vec<Key>, Error> {
        let res = self.execute_internal::<E>(query)?;

        Ok(res)
    }

    // execute
    pub fn execute<E: EntityKind>(self, query: DeleteQuery) -> Result<Vec<Key>, Error> {
        let res = self.execute_internal::<E>(query)?;

        Ok(res)
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(self, query: DeleteQuery) -> Result<Vec<Key>, DbError> {
        // validate query
        QueryValidate::<E>::validate(&query)?;

        // get store
        let store = self
            .data_registry
            .with(|db| db.try_get_store(E::Store::PATH))?;

        // resolve data keys
        let data_keys = self.resolve_data_keys::<E>(&query)?;

        // Get a single mutable borrow for the entire operation
        let mut deleted_rows = Vec::new();
        store.with_borrow_mut(|s| {
            for dk in data_keys {
                if let Some(data_value) = s.get(&dk) {
                    // remove from store
                    s.remove(&dk);

                    // if there are indexes we need to find and destroy them
                    if !E::INDEXES.is_empty() {
                        let entity: E = deserialize(&data_value.bytes)?;
                        self.remove_indexes::<E>(&entity)?;
                    }

                    // record deletion
                    deleted_rows.push(dk.key());
                }
            }

            Ok::<_, DbError>(())
        })?;

        // debug
        debug!(self.debug, "query.delete: deleted keys {deleted_rows:?}");

        Ok(deleted_rows)
    }

    fn resolve_data_keys<E: EntityKind>(
        &self,
        query: &DeleteQuery,
    ) -> Result<Vec<DataKey>, DbError> {
        // plan
        let planner = QueryPlanner::new(query.filter.as_ref());
        let plan = planner.plan::<E>();

        debug!(
            self.debug,
            "query.delete: query is {query:?}, plan is {plan:?}"
        );

        let store = self
            .data_registry
            .with(|db| db.try_get_store(E::Store::PATH))?;

        // 1) collect candidate keys from the plan (unfiltered superset)
        let mut keys: Vec<DataKey> = match plan {
            QueryPlan::Keys(keys) => keys,
            QueryPlan::Range(start, end) => store.with_borrow(|s| {
                s.range((Bound::Included(start.clone()), Bound::Included(end.clone())))
                    .map(|e| e.key().clone())
                    .collect()
            }),
            QueryPlan::Index(plan) => {
                let index = plan.index;
                let index_store = self
                    .index_registry
                    .with(|reg| reg.try_get_store(index.store))?;

                index_store.with_borrow(|istore| istore.resolve_data_keys::<E>(index, &plan.keys))
            }
        };

        // 2) apply filter (like LoadExecutor::finalize_rows) BEFORE limit
        if let Some(filter_expr) = &query.filter {
            let filter_simple = filter_expr.clone().simplify();

            // Borrow store immutably to evaluate rows
            keys = store.with_borrow(|s| {
                keys.into_iter()
                    .filter(|dk| {
                        s.get(dk).is_some_and(|data_value| {
                            // deserialize to E to expose FieldValues for FilterEvaluator
                            match crate::core::deserialize::<E>(&data_value.bytes) {
                                Ok(entity) => {
                                    let eval = FilterEvaluator::new(&entity);
                                    eval.eval(&filter_simple)
                                }
                                Err(_) => false,
                            }
                        })
                    })
                    .collect()
            });
        }

        // 3) apply limit AFTER filtering
        if let Some(limit_expr) = &query.limit
            && let Some(limit) = limit_expr.limit
        {
            keys.truncate(limit as usize);
        }

        Ok(keys)
    }

    // remove_indexes
    fn remove_indexes<E: EntityKind>(&self, entity: &E) -> Result<(), DbError> {
        for index in E::INDEXES {
            let store = self
                .index_registry
                .with(|reg| reg.try_get_store(index.store))?;

            store.with_borrow_mut(|this| {
                this.remove_index_entry(entity, index);
            });
        }

        Ok(())
    }
}
