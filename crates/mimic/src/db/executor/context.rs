use crate::{
    core::{
        Key,
        traits::{EntityKind, Path},
    },
    db::{
        Db, DbError,
        query::QueryPlan,
        store::{DataKey, DataRow, DataStore},
    },
};
use std::{marker::PhantomData, ops::Bound};

///
/// Context
///

pub struct Context<'a, E: EntityKind> {
    pub db: &'a Db<E::Canister>,
    _marker: PhantomData<E>,
}

impl<'a, E> Context<'a, E>
where
    E: EntityKind,
{
    #[must_use]
    pub const fn new(db: &'a Db<E::Canister>) -> Self {
        Self {
            db,
            _marker: PhantomData,
        }
    }

    pub fn with_store<R>(&self, f: impl FnOnce(&DataStore) -> R) -> Result<R, DbError> {
        self.db
            .with_data(|reg| reg.with_store(E::Store::PATH, f))
            .map_err(DbError::from)
    }

    pub fn with_store_mut<R>(&self, f: impl FnOnce(&mut DataStore) -> R) -> Result<R, DbError> {
        self.db
            .with_data(|reg| reg.with_store_mut(E::Store::PATH, f))
            .map_err(DbError::from)
    }

    ///
    /// Analyze Plan
    ///

    pub fn candidates_from_plan(&self, plan: QueryPlan) -> Result<Vec<DataKey>, DbError> {
        let candidates = match plan {
            QueryPlan::Keys(keys) => Self::to_data_keys(keys),

            QueryPlan::Range(start, end) => self.with_store(|s| {
                let start = Self::to_data_key(start);
                let end = Self::to_data_key(end);

                s.range((Bound::Included(start), Bound::Included(end)))
                    .map(|e| e.key().clone())
                    .collect()
            })?,

            QueryPlan::Index(index_plan) => {
                let index_store = self
                    .db
                    .with_index(|reg| reg.try_get_store(index_plan.index.store))?;

                index_store.with_borrow(|istore| {
                    istore.resolve_data_values::<E>(index_plan.index, &index_plan.values)
                })
            }
        };

        Ok(candidates)
    }

    pub fn rows_from_plan(&self, plan: QueryPlan) -> Result<Vec<DataRow>, DbError> {
        match plan {
            QueryPlan::Keys(keys) => {
                let data_keys = Self::to_data_keys(keys);
                self.load_many(&data_keys)
            }
            QueryPlan::Range(start, end) => {
                let start = Self::to_data_key(start);
                let end = Self::to_data_key(end);
                self.load_range(start, end)
            }
            QueryPlan::Index(_) => {
                let data_keys = self.candidates_from_plan(plan)?;
                self.load_many(&data_keys)
            }
        }
    }

    ///
    /// Load Helpers
    ///

    fn to_data_key(key: Key) -> DataKey {
        DataKey::new::<E>(key)
    }

    fn to_data_keys(keys: Vec<Key>) -> Vec<DataKey> {
        keys.into_iter().map(Self::to_data_key).collect()
    }

    fn load_many(&self, keys: &[DataKey]) -> Result<Vec<DataRow>, DbError> {
        self.with_store(|s| {
            keys.iter()
                .filter_map(|k| s.get(k).map(|entry| (k.clone(), entry)))
                .collect()
        })
    }

    fn load_range(&self, start: DataKey, end: DataKey) -> Result<Vec<DataRow>, DbError> {
        self.with_store(|s| {
            s.range((Bound::Included(start), Bound::Included(end)))
                .map(|e| (e.key().clone(), e.value()))
                .collect()
        })
    }
}
