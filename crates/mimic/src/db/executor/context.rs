use crate::{
    core::traits::{EntityKind, Path},
    db::{
        DbError,
        executor::FilterEvaluator,
        query::{FilterExpr, QueryPlan, QueryPlanner},
        store::{
            DataKey, DataRow, DataStoreLocal, DataStoreRegistryLocal, IndexStoreRegistryLocal,
        },
    },
    debug,
};
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
        let store = self.store::<E>()?;
        let candidates = match plan {
            QueryPlan::Keys(keys) => keys,

            QueryPlan::Range(start, end) => store.with_borrow(|s| {
                s.range((Bound::Included(start), Bound::Included(end)))
                    .map(|e| e.key().clone())
                    .collect()
            }),

            QueryPlan::Index(index_plan) => {
                let index_store = self
                    .index_registry
                    .with(|reg| reg.try_get_store(index_plan.index.store))?;
                index_store.with_borrow(|istore| {
                    istore.resolve_data_keys::<E>(index_plan.index, &index_plan.keys)
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
            keys.iter()
                .filter_map(|k| {
                    s.get(k).map(|entry| DataRow {
                        key: k.clone(),
                        entry,
                    })
                })
                .collect()
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

    #[must_use]
    pub fn filter_keys<E: EntityKind>(
        &self,
        store: &DataStoreLocal,
        keys: Vec<DataKey>,
        filter: &FilterExpr,
    ) -> Vec<DataKey> {
        let f = filter.clone().simplify();
        store.with_borrow(|s| {
            keys.into_iter()
                .filter(|dk| {
                    s.get(dk).is_some_and(|dv| {
                        crate::core::deserialize::<E>(&dv.bytes)
                            .ok()
                            .map(|e| FilterEvaluator::new(&e).eval(&f))
                            .unwrap_or(false)
                    })
                })
                .collect()
        })
    }

    pub fn paginate_rows<T>(rows: &mut Vec<T>, offset: u32, limit: Option<u32>) {
        let total = rows.len();
        let start = usize::min(offset as usize, total);
        let end = limit
            .map(|l| usize::min(start + l as usize, total))
            .unwrap_or(total);

        if start >= end {
            rows.clear();
        } else {
            rows.drain(..start);
            rows.truncate(end - start);
        }
    }

    #[must_use]
    pub fn truncate_keys(mut keys: Vec<DataKey>, limit: Option<u32>) -> Vec<DataKey> {
        if let Some(l) = limit {
            keys.truncate(l as usize);
        }
        keys
    }
}
