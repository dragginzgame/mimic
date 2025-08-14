use crate::{
    core::traits::{EntityKind, Path},
    db::{
        DbError,
        query::{FilterExpr, QueryPlan, QueryPlanner},
        store::{
            DataKey, DataRow, DataStoreLocal, DataStoreRegistryLocal, IndexStoreRegistryLocal,
        },
    },
};
use icu::debug;
use std::ops::Bound;

#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub data_registry: DataStoreRegistryLocal,
    pub index_registry: IndexStoreRegistryLocal,
    pub debug: bool,
}

impl Context {
    #[must_use]
    pub fn plan<E: EntityKind>(&self, filter: Option<&FilterExpr>) -> QueryPlan {
        let plan = QueryPlanner::new(filter).plan::<E>();
        debug!(self.debug, "query.plan: {plan:?}");
        plan
    }

    pub fn store<E: EntityKind>(&self) -> Result<DataStoreLocal, DbError> {
        self.data_registry
            .with(|r| r.try_get_store(E::Store::PATH))
            .map_err(DbError::from)
    }

    pub fn candidates_from_plan<E: EntityKind>(
        &self,
        plan: QueryPlan,
    ) -> Result<Vec<DataKey>, DbError> {
        let candidates = match plan {
            QueryPlan::Keys(keys) => keys,

            QueryPlan::Range(start, end) => {
                let store = self.store::<E>()?;

                store.with_borrow(|s| {
                    s.range((Bound::Included(start), Bound::Included(end)))
                        .map(|e| e.key().clone())
                        .collect()
                })
            }

            QueryPlan::Index(index_plan) => {
                let index_store = self
                    .index_registry
                    .with(|reg| reg.try_get_store(index_plan.index.store))?;

                index_store.with_borrow(|istore| {
                    istore.resolve_data_values::<E>(index_plan.index, &index_plan.values)
                })
            }
        };

        Ok(candidates)
    }

    pub fn rows_from_plan<E: EntityKind>(&self, plan: QueryPlan) -> Result<Vec<DataRow>, DbError> {
        let store = self.store::<E>()?;

        Ok(match plan {
            QueryPlan::Keys(keys) => self.load_many(&store, &keys),
            QueryPlan::Range(start, end) => self.load_range(&store, start, end),
            QueryPlan::Index(_) => {
                let keys = self.candidates_from_plan::<E>(plan)?;
                self.load_many(&store, &keys)
            }
        })
    }

    #[must_use]
    pub fn load_many(&self, store: &DataStoreLocal, keys: &[DataKey]) -> Vec<DataRow> {
        store.with_borrow(|s| {
            let mut out = Vec::with_capacity(keys.len());
            for k in keys {
                if let Some(entry) = s.get(k) {
                    out.push(DataRow {
                        key: k.clone(),
                        entry,
                    });
                }
            }
            out
        })
    }
    #[must_use]
    pub fn load_range(&self, store: &DataStoreLocal, start: DataKey, end: DataKey) -> Vec<DataRow> {
        store.with_borrow(|s| {
            s.range((Bound::Included(start), Bound::Included(end)))
                .map(|e| DataRow::new(e.key().clone(), e.value()))
                .collect()
        })
    }
}
