use crate::{
    core::{
        Key,
        traits::{EntityKind, Path},
    },
    db::{
        DbError,
        query::QueryPlan,
        store::{
            DataKey, DataRow, DataStoreLocal, DataStoreRegistryLocal, IndexStoreRegistryLocal,
        },
    },
};
use std::ops::Bound;

///
/// Context
///

#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub data_registry: DataStoreRegistryLocal,
    pub index_registry: IndexStoreRegistryLocal,
}

impl Context {
    fn to_data_key<E: EntityKind>(key: Key) -> DataKey {
        DataKey::new::<E>(key)
    }

    fn to_data_keys<E: EntityKind>(keys: Vec<Key>) -> Vec<DataKey> {
        keys.into_iter().map(|k| DataKey::new::<E>(k)).collect()
    }

    pub fn store<E: EntityKind>(&self) -> Result<DataStoreLocal, DbError> {
        self.data_registry
            .with(|r| r.try_get_store(E::Store::PATH))
            .map_err(DbError::from)
    }

    ///
    /// Analyze Plan
    ///

    pub fn candidates_from_plan<E: EntityKind>(
        &self,
        plan: QueryPlan,
    ) -> Result<Vec<DataKey>, DbError> {
        let candidates = match plan {
            QueryPlan::Keys(keys) => Self::to_data_keys::<E>(keys),

            QueryPlan::Range(start, end) => {
                let store = self.store::<E>()?;
                let start = Self::to_data_key::<E>(start);
                let end = Self::to_data_key::<E>(end);

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
            QueryPlan::Keys(keys) => {
                let data_keys = Self::to_data_keys::<E>(keys);
                self.load_many(&store, &data_keys)
            }
            QueryPlan::Range(start, end) => {
                let start = Self::to_data_key::<E>(start);
                let end = Self::to_data_key::<E>(end);
                self.load_range(&store, start, end)
            }
            QueryPlan::Index(_) => {
                let data_keys = self.candidates_from_plan::<E>(plan)?;
                self.load_many(&store, &data_keys)
            }
        })
    }

    ///
    /// Load Helpers
    ///

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
