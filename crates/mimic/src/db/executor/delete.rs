use crate::{
    Error,
    core::{
        Key, deserialize,
        traits::{EntityKind, FieldValue},
    },
    db::{
        Db, DbError,
        executor::{Context, FilterEvaluator},
        query::{
            DeleteQuery, FilterDsl, FilterExpr, FilterExt, QueryPlan, QueryPlanner, QueryValidate,
        },
    },
};
use std::marker::PhantomData;

///
/// DeleteExecutor
///

#[derive(Clone, Copy)]
pub struct DeleteExecutor<'a, E: EntityKind> {
    db: &'a Db<E::Canister>,
    debug: bool,
    _marker: PhantomData<E>,
}

impl<'a, E: EntityKind> DeleteExecutor<'a, E> {
    #[must_use]
    pub const fn from_db(db: &'a Db<E::Canister>) -> Self {
        Self {
            db,
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

    pub fn one(self, value: impl FieldValue) -> Result<Vec<Key>, Error> {
        let query = DeleteQuery::new().one::<E>(value);
        self.execute(&query)
    }

    pub fn many(
        self,
        values: impl IntoIterator<Item = impl FieldValue>,
    ) -> Result<Vec<Key>, Error> {
        let query = DeleteQuery::new().many::<E>(values);
        self.execute(&query)
    }

    pub fn all(self) -> Result<Vec<Key>, Error> {
        let query = DeleteQuery::new();
        self.execute(&query)
    }

    pub fn filter(self, f: impl FnOnce(FilterDsl) -> FilterExpr) -> Result<Vec<Key>, Error> {
        let query = DeleteQuery::new().filter(f);
        self.execute(&query)
    }

    ///
    /// EXECUTION PREP
    ///

    const fn context(&self) -> Context<'_, E> {
        Context::new(self.db)
    }

    // plan
    // chatgpt says cleaner to keep it a method
    #[allow(clippy::unused_self)]
    fn plan(&self, query: &DeleteQuery) -> QueryPlan {
        QueryPlanner::new(query.filter.as_ref()).plan::<E>()
    }

    ///
    /// EXECUTION METHODS
    ///

    // explain
    pub fn explain(self, query: &DeleteQuery) -> Result<QueryPlan, Error> {
        QueryValidate::<E>::validate(query).map_err(DbError::from)?;

        Ok(self.plan(query))
    }

    // response
    // for the automated query endpoint, we will make this more flexible in the future
    pub fn response(self, query: &DeleteQuery) -> Result<Vec<Key>, Error> {
        let res = self.execute(query)?;

        Ok(res)
    }

    // execute
    pub fn execute(self, query: &DeleteQuery) -> Result<Vec<Key>, Error> {
        QueryValidate::<E>::validate(query).map_err(DbError::from)?;

        let ctx = self.context();
        let plan = self.plan(query);
        let keys = ctx.candidates_from_plan(plan)?; // no deserialization here

        // query prep
        let limit = query
            .limit
            .as_ref()
            .and_then(|l| l.limit)
            .map(|l| l as usize);
        let filter_simplified = query.filter.as_ref().map(|f| f.clone().simplify());

        let mut deleted_rows: Vec<Key> = Vec::new();
        ctx.with_store_mut(|s| {
            for dk in keys {
                // If we already hit the limit, bail early
                if let Some(max) = limit
                    && deleted_rows.len() >= max
                {
                    break;
                }

                // Peek the value once
                let Some(bytes) = s.get(&dk) else {
                    continue;
                };

                // Decide if we need to deserialize:
                // - Needed if we have a filter (to evaluate)
                // - Or if we *might* delete and need to drop index entries
                let mut entity_opt: Option<E> = None;

                // Evaluate filter if present
                if let Some(ref f) = filter_simplified {
                    // deserialize once to evaluate
                    match deserialize::<E>(&bytes) {
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
                        None => deserialize::<E>(&bytes)?,
                    };
                    self.remove_indexes(&ent)?;
                }

                deleted_rows.push(dk.key());
            }

            Ok::<_, DbError>(())
        })??;

        //   debug!(self.debug, "query.delete: deleted keys {deleted_rows:?}");

        Ok(deleted_rows)
    }

    // remove_indexes
    fn remove_indexes(&self, entity: &E) -> Result<(), DbError> {
        for index in E::INDEXES {
            let store = self.db.with_index(|reg| reg.try_get_store(index.store))?;

            store.with_borrow_mut(|this| {
                this.remove_index_entry(entity, index);
            });
        }

        Ok(())
    }
}
