use crate::{
    MimicError,
    core::{
        Key, Value, deserialize,
        traits::{EntityKind, IndexKindTuple, Path},
    },
    db::{
        DbError,
        executor::IndexAction,
        query::{DeleteQuery, QueryPlan, QueryPlanner},
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

    // one
    // helper method, creates query
    pub fn one<E: EntityKind>(&self, value: impl Into<Value>) -> Result<Vec<Key>, MimicError> {
        self.execute::<E>(DeleteQuery::new().one::<E>(value))
    }

    // many
    // helper method, creates query
    pub fn many<E, I>(&self, values: I) -> Result<Vec<Key>, MimicError>
    where
        E: EntityKind,
        I: IntoIterator,
        I::Item: Into<Value>,
    {
        self.execute::<E>(DeleteQuery::new().many::<E, I>(values))
    }

    // all
    pub fn all<E: EntityKind>(&self) -> Result<Vec<Key>, MimicError> {
        self.execute::<E>(DeleteQuery::new())
    }

    // filter_eq
    pub fn filter_eq<E: EntityKind>(
        self,
        field: &str,
        value: impl Into<Value>,
    ) -> Result<Vec<Key>, MimicError> {
        self.execute::<E>(DeleteQuery::new().filter_eq(field, value))
    }

    ///
    /// EXECUTION METHODS
    ///

    // execute
    pub fn execute<E: EntityKind>(self, query: DeleteQuery) -> Result<Vec<Key>, MimicError> {
        let res = self.execute_internal::<E>(query)?;

        Ok(res)
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(self, query: DeleteQuery) -> Result<Vec<Key>, DbError> {
        let planner = QueryPlanner::new(query.filter.as_ref());
        let plan = planner.plan::<E>();

        debug!(
            self.debug,
            "query.delete: query is {query:?}, plan is {plan:?}"
        );

        // get store
        let store = self
            .data_registry
            .with(|db| db.try_get_store(E::Store::PATH))?;

        // get data keys
        let data_keys: Vec<DataKey> = match &plan {
            QueryPlan::Keys(keys) => keys.clone(),
            QueryPlan::Range(start, end) => store.with_borrow(|store| {
                store
                    .range((Bound::Included(start.clone()), Bound::Included(end.clone())))
                    .map(|entry| entry.key().clone())
                    .collect()
            }),

            QueryPlan::Index(index_plan) => {
                // get the index store
                let index_store = self
                    .index_registry
                    .with(|reg| reg.try_get_store(index_plan.store_path))?;

                // resolve keys
                index_store.with_borrow(|istore| {
                    istore.resolve_data_keys::<E>(
                        index_plan.index_path,
                        index_plan.index_fields,
                        &index_plan.keys,
                    )
                })
            }
        };

        // Get a single mutable borrow for the entire operation
        let mut deleted_rows = Vec::new();
        store.with_borrow_mut(|s| {
            for dk in data_keys {
                if let Some(data_value) = s.get(&dk) {
                    // remove from store
                    s.remove(&dk);

                    // if there are indexes we need to find and destroy them
                    if E::Indexes::HAS_INDEXES {
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

    // remove_indexes
    fn remove_indexes<E: EntityKind>(&self, entity: &E) -> Result<(), DbError> {
        let mut action = IndexAction::Remove {
            entity,
            registry: &self.index_registry,
        };

        E::Indexes::for_each(&mut action)
    }
}
