use crate::{
    Error,
    core::{Key, Value, deserialize, traits::EntityKind},
    db::{
        Db, DbError,
        executor::{Context, FilterEvaluator},
        query::{
            FilterDsl, FilterExpr, FilterExt, IntoFilterOpt, LoadQuery, QueryPlan, QueryPlanner,
            QueryValidate, SortDirection, SortExpr,
        },
        response::LoadCollection,
    },
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

    pub fn one(&self, value: impl Into<Value>) -> Result<E, Error> {
        let query = LoadQuery::new().one::<E>(value);
        self.execute(&query)?.try_entity()
    }

    pub fn many(
        &self,
        values: impl IntoIterator<Item = impl Into<Value>>,
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

    // plan
    // chatgpt says cleaner to keep it a method
    #[allow(clippy::unused_self)]
    fn plan(&self, query: &LoadQuery) -> QueryPlan {
        QueryPlanner::new(query.filter.as_ref()).plan::<E>()
    }

    ///
    /// EXECUTION METHODS
    ///

    // explain
    pub fn explain(self, query: &LoadQuery) -> Result<QueryPlan, Error> {
        QueryValidate::<E>::validate(query).map_err(DbError::from)?;

        Ok(self.plan(query))
    }

    // response
    // for the automated query endpoint, we will make this more flexible in the future
    pub fn response(&self, query: &LoadQuery) -> Result<Vec<Key>, Error> {
        let res = self.execute(query)?.keys();

        Ok(res)
    }

    /// Execute a full query and return a collection of entities.
    pub fn execute(&self, query: &LoadQuery) -> Result<LoadCollection<E>, Error> {
        QueryValidate::<E>::validate(query).map_err(DbError::from)?;

        let ctx = self.context();
        let plan = self.plan(query);

        // Convert data rows -> entity rows
        let mut rows: Vec<(Key, E)> = ctx
            .rows_from_plan(plan)?
            .into_iter()
            .map(|(k, v)| deserialize::<E>(&v).map(|entry| (k.key(), entry)))
            .collect::<Result<_, _>>()?;

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
        if let Some(lim) = &query.limit {
            Self::apply_pagination(&mut rows, lim.offset, lim.limit);
        }

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
