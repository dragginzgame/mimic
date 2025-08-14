use crate::{
    Error,
    core::{Key, Value, deserialize, traits::EntityKind},
    db::{
        DbError,
        executor::{Context, FilterEvaluator},
        query::{DeleteQuery, FilterDsl, FilterExpr, FilterExt, QueryValidate},
        store::{DataStoreRegistryLocal, IndexStoreRegistryLocal},
    },
};
use icu::debug;
use std::marker::PhantomData;

///
/// DeleteExecutor
///

#[derive(Clone, Copy, Debug)]
pub struct DeleteExecutor<E: EntityKind> {
    data_registry: DataStoreRegistryLocal,
    index_registry: IndexStoreRegistryLocal,
    debug: bool,
    _marker: PhantomData<E>,
}

impl<E: EntityKind> DeleteExecutor<E> {
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
            _marker: PhantomData,
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

    pub fn one(self, value: impl Into<Value>) -> Result<Vec<Key>, Error> {
        self.execute(DeleteQuery::new().one::<E>(value))
    }

    pub fn many(
        self,
        values: impl IntoIterator<Item = impl Into<Value>>,
    ) -> Result<Vec<Key>, Error> {
        self.execute(DeleteQuery::new().many::<E, _>(values))
    }

    pub fn all(self) -> Result<Vec<Key>, Error> {
        self.execute(DeleteQuery::new())
    }

    pub fn filter(self, f: impl FnOnce(FilterDsl) -> FilterExpr) -> Result<Vec<Key>, Error> {
        self.execute(DeleteQuery::new().filter(f))
    }

    ///
    /// EXECUTION METHODS
    ///

    const fn context(&self) -> Context {
        Context {
            data_registry: self.data_registry,
            index_registry: self.index_registry,
            debug: self.debug,
        }
    }

    // response
    // for the automated query endpoint, we will make this more flexible in the future
    pub fn response(self, query: DeleteQuery) -> Result<Vec<Key>, Error> {
        let res = self.execute(query)?;

        Ok(res)
    }

    // execute
    pub fn execute(self, query: DeleteQuery) -> Result<Vec<Key>, Error> {
        QueryValidate::<E>::validate(&query).map_err(DbError::from)?;

        let ctx = self.context();
        let plan = ctx.plan::<E>(query.filter.as_ref());
        let store = ctx.store::<E>()?;
        let keys = ctx.candidates_from_plan::<E>(plan)?; // no deserialization here

        let limit = query
            .limit
            .as_ref()
            .and_then(|l| l.limit)
            .map(|l| l as usize);
        let filter_simplified = query.filter.as_ref().map(|f| f.clone().simplify());

        let mut deleted_rows: Vec<Key> = Vec::new();

        store.with_borrow_mut(|s| {
            for dk in keys {
                // If we already hit the limit, bail early
                if let Some(max) = limit
                    && deleted_rows.len() >= max
                {
                    break;
                }

                // Peek the value once
                let Some(data_value) = s.get(&dk) else {
                    continue;
                };

                // Decide if we need to deserialize:
                // - Needed if we have a filter (to evaluate)
                // - Or if we *might* delete and need to drop index entries
                let mut entity_opt: Option<E> = None;

                // Evaluate filter if present
                if let Some(ref f) = filter_simplified {
                    // deserialize once to evaluate
                    match deserialize::<E>(&data_value.bytes) {
                        Ok(ent) => {
                            if !FilterEvaluator::new(&ent).eval(f) {
                                continue; // not matched; skip
                            }
                            entity_opt = Some(ent); // reuse for index removal
                        }
                        Err(_) => continue,
                    }
                }

                // Passed filter (or no filter) → delete
                s.remove(&dk);

                // Remove indexes if any. Only deserialize if we haven't yet and need it.
                if !E::INDEXES.is_empty() {
                    let ent = match entity_opt {
                        Some(ent) => ent,
                        None => deserialize::<E>(&data_value.bytes)?,
                    };
                    self.remove_indexes(&ent)?;
                }

                deleted_rows.push(dk.key());
            }

            Ok::<_, DbError>(())
        })?;

        debug!(self.debug, "query.delete: deleted keys {deleted_rows:?}");

        Ok(deleted_rows)
    }

    // remove_indexes
    fn remove_indexes(&self, entity: &E) -> Result<(), DbError> {
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
