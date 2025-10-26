use crate::{
    Error,
    core::{
        Key,
        traits::{EntityKind, FieldValue},
    },
    db::{
        Db,
        executor::{FilterEvaluator, plan_for},
        query::{
            FilterDsl, FilterExpr, FilterExt, IntoFilterOpt, LoadQuery, QueryPlan, QueryValidate,
            SortDirection, SortExpr,
        },
        response::Response,
    },
    obs::metrics,
};
use canic::{Log, log};
use std::marker::PhantomData;

///
/// LoadExecutor
///

#[derive(Clone, Copy)]
pub struct LoadExecutor<E: EntityKind> {
    db: Db<E::Canister>,
    debug: bool,
    _marker: PhantomData<E>,
}

impl<E: EntityKind> LoadExecutor<E> {
    #[must_use]
    pub const fn new(db: Db<E::Canister>, debug: bool) -> Self {
        Self {
            db,
            debug,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn debug_log(&self, s: impl Into<String>) {
        if self.debug {
            log!(Log::Debug, "{}", s.into());
        }
    }

    //
    // HELPER METHODS
    // these will create an intermediate query
    //

    pub fn one(&self, value: impl FieldValue) -> Result<E, Error> {
        let query = LoadQuery::new().one::<E>(value);
        self.execute(query)?.try_entity()
    }

    pub fn only(&self) -> Result<E, Error> {
        let query = LoadQuery::new().one::<E>(());
        self.execute(query)?.try_entity()
    }

    pub fn many(
        &self,
        values: impl IntoIterator<Item = impl FieldValue>,
    ) -> Result<Response<E>, Error> {
        let query = LoadQuery::new().many::<E>(values);
        self.execute(query)
    }

    pub fn all(&self) -> Result<Response<E>, Error> {
        let query = LoadQuery::new();
        self.execute(query)
    }

    pub fn filter<F, R>(self, f: F) -> Result<Response<E>, Error>
    where
        F: FnOnce(FilterDsl) -> R,
        R: IntoFilterOpt,
    {
        let query = LoadQuery::new().filter(f);
        self.execute(query)
    }

    pub fn count_all(self) -> Result<u32, Error> {
        let query = LoadQuery::all();
        self.count(query)
    }

    ///
    /// EXECUTION METHODS
    ///

    // explain
    pub fn explain(self, query: LoadQuery) -> Result<QueryPlan, Error> {
        QueryValidate::<E>::validate(&query)?;

        Ok(plan_for::<E>(query.filter.as_ref()))
    }

    /// Execute a full query and return a collection of entities.
    pub fn execute(&self, query: LoadQuery) -> Result<Response<E>, Error> {
        let mut span = metrics::Span::<E>::new(metrics::ExecKind::Load);
        QueryValidate::<E>::validate(&query)?;

        self.debug_log(format!("🧭 Executing query: {:?} on {}", query, E::PATH));

        let ctx = self.db.context::<E>();
        let plan = plan_for::<E>(query.filter.as_ref());

        self.debug_log(format!("📄 Query plan: {plan:?}"));

        // Fast path: pre-pagination
        let pre_paginated = query.filter.is_none() && query.sort.is_none() && query.limit.is_some();
        let data_rows = if pre_paginated {
            let lim = query.limit.as_ref().unwrap();
            ctx.rows_from_plan_with_pagination(plan, lim.offset, lim.limit)?
        } else {
            ctx.rows_from_plan(plan)?
        };

        self.debug_log(format!(
            "📦 Loaded {} data rows before deserialization",
            data_rows.len()
        ));

        // Deserialize
        let mut rows: Vec<(Key, E)> = ctx.deserialize_rows(data_rows)?;
        self.debug_log(format!(
            "🧩 Deserialized {} entities before filtering",
            rows.len()
        ));

        // Filtering
        if let Some(f) = &query.filter {
            let simplified = f.clone().simplify();
            Self::apply_filter(&mut rows, &simplified);

            self.debug_log(format!(
                "🔎 Applied filter -> {} entities remaining",
                rows.len()
            ));
        }

        // Sorting
        if let Some(sort) = &query.sort
            && rows.len() > 1
        {
            Self::apply_sort(&mut rows, sort);
            self.debug_log("↕️ Applied sort expression");
        }

        // Pagination
        if let Some(lim) = &query.limit
            && !pre_paginated
        {
            apply_pagination(&mut rows, lim.offset, lim.limit);
            self.debug_log(format!(
                "📏 Applied pagination (offset={}, limit={:?}) -> {} entities",
                lim.offset,
                lim.limit,
                rows.len()
            ));
        }

        crate::db::executor::set_rows_from_len(&mut span, rows.len());
        self.debug_log(format!("✅ Query complete -> {} final rows", rows.len()));

        Ok(Response(rows))
    }

    /// currently just doing the same as execute()
    /// keeping it separate in case we can optimise count queries in the future
    #[allow(clippy::cast_possible_truncation)]
    pub fn count(self, query: LoadQuery) -> Result<u32, Error> {
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
}

/// Apply offset/limit pagination to an in-memory vector, in-place.
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

#[cfg(test)]
mod tests {
    use super::apply_pagination;

    #[test]
    fn pagination_empty_vec() {
        let mut v: Vec<i32> = vec![];
        apply_pagination(&mut v, 0, Some(10));
        assert!(v.is_empty());
    }

    #[test]
    fn pagination_offset_beyond_len_clears() {
        let mut v = vec![1, 2, 3];
        apply_pagination(&mut v, 10, Some(5));
        assert!(v.is_empty());
    }

    #[test]
    fn pagination_no_limit_from_offset() {
        let mut v = vec![1, 2, 3, 4, 5];
        apply_pagination(&mut v, 2, None);
        assert_eq!(v, vec![3, 4, 5]);
    }

    #[test]
    fn pagination_exact_window() {
        let mut v = vec![10, 20, 30, 40, 50];
        // offset 1, limit 3 -> elements [20,30,40]
        apply_pagination(&mut v, 1, Some(3));
        assert_eq!(v, vec![20, 30, 40]);
    }

    #[test]
    fn pagination_limit_exceeds_tail() {
        let mut v = vec![10, 20, 30];
        // offset 1, limit large -> [20,30]
        apply_pagination(&mut v, 1, Some(999));
        assert_eq!(v, vec![20, 30]);
    }
}
