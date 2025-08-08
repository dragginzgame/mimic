use crate::{
    Error,
    core::{Key, Value, traits::EntityKind},
    db::{
        DbError,
        executor::{Context, FilterEvaluator},
        query::{FilterBuilder, FilterExpr, LoadQuery, QueryValidate, SortDirection, SortExpr},
        response::{EntityRow, LoadCollection},
        store::{DataStoreRegistryLocal, IndexStoreRegistryLocal},
    },
};

///
/// LoadExecutor
///

#[derive(Clone, Copy, Debug)]
pub struct LoadExecutor {
    data_registry: DataStoreRegistryLocal,
    index_registry: IndexStoreRegistryLocal,
    debug: bool,
}

impl LoadExecutor {
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

    pub fn one<E: EntityKind>(&self, value: impl Into<Value>) -> Result<E, Error> {
        self.execute::<E>(LoadQuery::new().one::<E>(value))?
            .try_entity()
    }

    pub fn many<E: EntityKind>(
        &self,
        values: impl IntoIterator<Item = impl Into<Value>>,
    ) -> Result<LoadCollection<E>, Error> {
        self.execute::<E>(LoadQuery::new().many::<E>(values))
    }

    pub fn all<E: EntityKind>(&self) -> Result<LoadCollection<E>, Error> {
        self.execute::<E>(LoadQuery::new())
    }

    pub fn filter<E: EntityKind>(
        &self,
        f: impl FnOnce(FilterBuilder) -> FilterBuilder,
    ) -> Result<LoadCollection<E>, Error> {
        self.execute::<E>(LoadQuery::new().filter(f))
    }

    pub fn count_all<E: EntityKind>(self) -> Result<u32, Error> {
        self.count::<E>(LoadQuery::all())
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
    pub fn response<E: EntityKind>(&self, query: LoadQuery) -> Result<Vec<Key>, Error> {
        let res = self.execute::<E>(query)?.keys();

        Ok(res)
    }

    /// Execute a full query and return a collection of entities.
    pub fn execute<E: EntityKind>(&self, query: LoadQuery) -> Result<LoadCollection<E>, Error> {
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
    pub fn count<E: EntityKind>(self, query: LoadQuery) -> Result<u32, Error> {
        let count = self.execute::<E>(query)?.count();

        Ok(count)
    }

    // apply_filter
    fn apply_filter<E: EntityKind>(rows: &mut Vec<EntityRow<E>>, filter: &FilterExpr) {
        rows.retain(|row| FilterEvaluator::new(&row.entry.entity).eval(filter));
    }

    // apply_sort
    fn apply_sort<E: EntityKind>(rows: &mut [EntityRow<E>], sort_expr: &SortExpr) {
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
