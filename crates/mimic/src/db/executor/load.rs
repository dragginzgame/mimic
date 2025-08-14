use crate::{
    Error,
    core::{Key, Value, traits::EntityKind},
    db::{
        DbError,
        executor::{Context, FilterEvaluator},
        query::{
            FilterDsl, FilterExpr, FilterExt, IntoFilterOpt, LoadQuery, QueryValidate,
            SortDirection, SortExpr,
        },
        response::{EntityRow, LoadCollection},
        store::{DataStoreRegistryLocal, IndexStoreRegistryLocal},
    },
};
use std::marker::PhantomData;

///
/// LoadExecutor
///

#[derive(Clone, Copy, Debug)]
pub struct LoadExecutor<E: EntityKind> {
    data_registry: DataStoreRegistryLocal,
    index_registry: IndexStoreRegistryLocal,
    debug: bool,
    _marker: PhantomData<E>,
}

impl<E: EntityKind> LoadExecutor<E> {
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

    //
    // HELPER METHODS
    // these will create an intermediate query
    //

    pub fn one(&self, value: impl Into<Value>) -> Result<E, Error> {
        self.execute(LoadQuery::new().one::<E>(value))?.try_entity()
    }

    pub fn many(
        &self,
        values: impl IntoIterator<Item = impl Into<Value>>,
    ) -> Result<LoadCollection<E>, Error> {
        self.execute(LoadQuery::new().many::<E, _>(values))
    }

    pub fn all(&self) -> Result<LoadCollection<E>, Error> {
        self.execute(LoadQuery::new())
    }

    pub fn filter<F, R>(self, f: F) -> Result<LoadCollection<E>, Error>
    where
        F: FnOnce(FilterDsl) -> R,
        R: IntoFilterOpt,
    {
        self.execute(LoadQuery::new().filter(f))
    }

    pub fn count_all(self) -> Result<u32, Error> {
        self.count(LoadQuery::all())
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
    pub fn response(&self, query: LoadQuery) -> Result<Vec<Key>, Error> {
        let res = self.execute(query)?.keys();

        Ok(res)
    }

    /// Execute a full query and return a collection of entities.
    pub fn execute(&self, query: LoadQuery) -> Result<LoadCollection<E>, Error> {
        QueryValidate::<E>::validate(&query).map_err(DbError::from)?;

        let ctx = self.context();
        let plan = ctx.plan::<E>(query.filter.as_ref());
        let rows = ctx.rows_from_plan::<E>(plan)?;

        let mut entities: Vec<_> = rows
            .into_iter()
            .map(EntityRow::<E>::try_from)
            .collect::<Result<_, _>>()
            .map_err(DbError::from)?;

        if let Some(f) = &query.filter {
            Self::apply_filter(&mut entities, &f.clone().simplify());
        }

        if let Some(sort) = &query.sort
            && entities.len() > 1
        {
            Self::apply_sort(&mut entities, sort);
        }

        if let Some(lim) = &query.limit {
            Self::apply_pagination(&mut entities, lim.offset, lim.limit);
        }

        Ok(LoadCollection(entities))
    }

    /// currently just doing the same as execute()
    /// keeping it separate in case we can optimise count queries in the future
    #[allow(clippy::cast_possible_truncation)]
    pub fn count(self, query: LoadQuery) -> Result<u32, Error> {
        let count = self.execute(query)?.count();

        Ok(count)
    }

    // apply_filter
    fn apply_filter(rows: &mut Vec<EntityRow<E>>, filter: &FilterExpr) {
        rows.retain(|row| FilterEvaluator::new(&row.entry.entity).eval(filter));
    }

    // apply_sort
    fn apply_sort(rows: &mut [EntityRow<E>], sort_expr: &SortExpr) {
        rows.sort_by(|a, b| {
            for (field, direction) in sort_expr.iter() {
                let (Some(va), Some(vb)) = (
                    a.entry.entity.get_value(field),
                    b.entry.entity.get_value(field),
                ) else {
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
