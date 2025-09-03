use crate::{
    Error,
    core::{
        Key,
        traits::{EntityKind, FieldValue},
    },
    db::{
        Db, DbError,
        executor::{Context, FilterEvaluator, plan_for},
        query::{
            FilterDsl, FilterExpr, FilterExt, IntoFilterOpt, LoadQuery, QueryPlan, QueryValidate,
            SortDirection, SortExpr,
        },
        response::LoadCollection,
    },
    metrics,
};
use std::marker::PhantomData;

///
/// LoadExecutor
///

#[derive(Clone, Copy)]
pub struct LoadExecutor<'a, E: EntityKind> {
    db: &'a Db<E::Canister>,
    debug: bool,
    _marker: PhantomData<E>,
}

impl<'a, E: EntityKind> LoadExecutor<'a, E> {
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

    //
    // HELPER METHODS
    // these will create an intermediate query
    //

    pub fn one(&self, value: impl FieldValue) -> Result<E, Error> {
        let query = LoadQuery::new().one::<E>(value);
        self.execute(&query)?.try_entity()
    }

    pub fn many(
        &self,
        values: impl IntoIterator<Item = impl FieldValue>,
    ) -> Result<LoadCollection<E>, Error> {
        let query = LoadQuery::new().many::<E>(values);
        self.execute(&query)
    }

    pub fn all(&self) -> Result<LoadCollection<E>, Error> {
        let query = LoadQuery::new();
        self.execute(&query)
    }

    pub fn filter<F, R>(self, f: F) -> Result<LoadCollection<E>, Error>
    where
        F: FnOnce(FilterDsl) -> R,
        R: IntoFilterOpt,
    {
        let query = LoadQuery::new().filter(f);
        self.execute(&query)
    }

    pub fn count_all(self) -> Result<u32, Error> {
        let query = LoadQuery::all();
        self.count(&query)
    }

    ///
    /// EXECUTION PREP
    ///

    const fn context(&self) -> Context<'_, E> {
        Context::new(self.db)
    }

    ///
    /// EXECUTION METHODS
    ///

    // explain
    pub fn explain(self, query: &LoadQuery) -> Result<QueryPlan, Error> {
        QueryValidate::<E>::validate(query).map_err(DbError::from)?;

        Ok(plan_for::<E>(query.filter.as_ref()))
    }

    // response used by automated query endpoints
    pub fn response(&self, query: &LoadQuery) -> Result<Vec<Key>, Error> {
        let res = self.execute(query)?.keys();

        Ok(res)
    }

    /// Execute a full query and return a collection of entities.
    pub fn execute(&self, query: &LoadQuery) -> Result<LoadCollection<E>, Error> {
        let mut span = metrics::Span::<E>::new(metrics::ExecKind::Load);
        QueryValidate::<E>::validate(query).map_err(DbError::from)?;

        let ctx = self.context();
        let plan = plan_for::<E>(query.filter.as_ref());

        // Fast path: if there is no filter or sort, and a limit is specified,
        // apply pagination at the storage layer to avoid deserializing discarded rows.
        let pre_paginated = query.filter.is_none() && query.sort.is_none() && query.limit.is_some();
        let data_rows = if pre_paginated {
            let lim = query.limit.as_ref().unwrap();
            ctx.rows_from_plan_with_pagination(plan, lim.offset, lim.limit)?
        } else {
            ctx.rows_from_plan(plan)?
        };

        // Convert data rows -> entity rows
        let mut rows: Vec<(Key, E)> = ctx.deserialize_rows(data_rows)?;

        // Filtering
        if let Some(f) = &query.filter {
            Self::apply_filter(&mut rows, &f.clone().simplify());
        }

        // Sorting
        if let Some(sort) = &query.sort
            && rows.len() > 1
        {
            Self::apply_sort(&mut rows, sort);
        }

        // Pagination
        if let Some(lim) = &query.limit
            && !pre_paginated
        {
            Self::apply_pagination(&mut rows, lim.offset, lim.limit);
        }

        crate::db::executor::set_rows_from_len(&mut span, rows.len());

        Ok(LoadCollection(rows))
    }

    /// currently just doing the same as execute()
    /// keeping it separate in case we can optimise count queries in the future
    #[allow(clippy::cast_possible_truncation)]
    pub fn count(self, query: &LoadQuery) -> Result<u32, Error> {
        let count = self.execute(query)?.count();

        Ok(count)
    }

    // apply_filter
    fn apply_filter(rows: &mut Vec<(Key, E)>, filter: &FilterExpr) {
        rows.retain(|(_, e)| FilterEvaluator::new(e).eval(filter));
    }

    // apply_sort
    fn apply_sort(rows: &mut [(Key, E)], sort_expr: &SortExpr) {
        rows.sort_by(|(_, ea), (_, eb)| {
            for (field, direction) in sort_expr.iter() {
                let (Some(va), Some(vb)) = (ea.get_value(field), eb.get_value(field)) else {
                    continue;
                };

                if let Some(ordering) = va.partial_cmp(&vb) {
                    return match direction {
                        SortDirection::Asc => ordering,
                        SortDirection::Desc => ordering.reverse(),
                    };
                }
            }
            core::cmp::Ordering::Equal
        });
    }

    pub fn apply_pagination<T>(rows: &mut Vec<T>, offset: u32, limit: Option<u32>) {
        let total = rows.len();
        let start = usize::min(offset as usize, total);
        let end = limit.map_or(total, |l| usize::min(start + l as usize, total));

        if start >= end {
            rows.clear();
        } else {
            rows.drain(..start);
            rows.truncate(end - start);
        }
    }
}
