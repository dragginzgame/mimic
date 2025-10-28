use crate::{
    Error,
    core::{
        Key, deserialize,
        traits::{EntityKind, FieldValue},
    },
    db::{
        Db,
        executor::FilterEvaluator,
        query::{DeleteQuery, FilterDsl, FilterExpr, FilterExt, QueryPlan, QueryValidate},
        response::Response,
    },
    obs::metrics,
};
use std::marker::PhantomData;

///
/// DeleteExecutor
///

#[derive(Clone, Copy)]
pub struct DeleteExecutor<E: EntityKind> {
    db: Db<E::Canister>,
    debug: bool,
    _marker: PhantomData<E>,
}

impl<E: EntityKind> DeleteExecutor<E> {
    #[must_use]
    pub const fn new(db: Db<E::Canister>, debug: bool) -> Self {
        Self {
            db,
            debug,
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
    /// one and only skip the intermediate collection data structure
    /// and error out if a row cannot be found
    ///

    pub fn one(self, value: impl FieldValue) -> Result<Key, Error> {
        let query = DeleteQuery::new().one::<E>(value);
        self.execute(query)?.try_key()
    }

    pub fn only(self) -> Result<Key, Error> {
        let query = DeleteQuery::new().one::<E>(());
        self.execute(query)?.try_key()
    }

    pub fn many(
        self,
        values: impl IntoIterator<Item = impl FieldValue>,
    ) -> Result<Response<E>, Error> {
        let query = DeleteQuery::new().many::<E>(values);
        self.execute(query)
    }

    pub fn all(self) -> Result<Response<E>, Error> {
        let query = DeleteQuery::new();
        self.execute(query)
    }

    pub fn filter(self, f: impl FnOnce(FilterDsl) -> FilterExpr) -> Result<Response<E>, Error> {
        let query = DeleteQuery::new().filter(f);
        self.execute(query)
    }

    ///
    /// EXECUTION METHODS
    ///

    // explain
    pub fn explain(self, query: DeleteQuery) -> Result<QueryPlan, Error> {
        QueryValidate::<E>::validate(&query)?;

        Ok(crate::db::executor::plan_for::<E>(query.filter.as_ref()))
    }

    // execute
    pub fn execute(self, query: DeleteQuery) -> Result<Response<E>, Error> {
        let mut span = metrics::Span::<E>::new(metrics::ExecKind::Delete);
        QueryValidate::<E>::validate(&query)?;

        let ctx = self.db.context::<E>();
        let plan = crate::db::executor::plan_for::<E>(query.filter.as_ref());
        let keys = ctx.candidates_from_plan(plan)?; // no deserialization here

        // query prep
        let limit = query
            .limit
            .as_ref()
            .and_then(|l| l.limit)
            .map(|l| l as usize);
        let filter_simplified = query.filter.as_ref().map(|f| f.clone().simplify());

        let mut res: Vec<(Key, E)> = Vec::with_capacity(limit.unwrap_or(0));
        ctx.with_store_mut(|s| {
            for dk in keys {
                // early limit
                if let Some(max) = limit
                    && res.len() >= max
                {
                    break;
                }

                // read value
                let Some(bytes) = s.get(&dk) else {
                    continue;
                };

                // deserialize once
                let Ok(entity) = deserialize::<E>(&bytes) else {
                    continue;
                };

                // filter check
                if let Some(ref f) = filter_simplified
                    && !FilterEvaluator::new(&entity).eval(f)
                {
                    continue;
                }

                // delete row and remove indexes
                s.remove(&dk);
                if !E::INDEXES.is_empty() {
                    self.remove_indexes(&entity)?;
                }

                // store result (key + deleted entity)
                res.push((dk.key(), entity));
            }

            Ok::<_, Error>(())
        })??;

        //   canic::cdk::println!("query.delete: deleted keys {deleted_rows:?}");

        crate::db::executor::set_rows_from_len(&mut span, res.len());

        Ok(Response(res))
    }

    // remove_indexes
    fn remove_indexes(&self, entity: &E) -> Result<(), Error> {
        for index in E::INDEXES {
            let store = self.db.with_index(|reg| reg.try_get_store(index.store))?;

            store.with_borrow_mut(|this| {
                this.remove_index_entry(entity, index);
            });
        }

        Ok(())
    }
}
