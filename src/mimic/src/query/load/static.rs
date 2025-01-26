use crate::{
    db::{types::EntityRow, Db},
    orm::traits::Entity,
    query::{
        load::{ELoadResult, LoadError, Loader},
        types::{Filter, LoadMethod, Order},
        DebugContext, Resolver,
    },
};
use std::marker::PhantomData;

///
/// ELoadBuilder
///

#[derive(Default)]
pub struct ELoadBuilder<E>
where
    E: Entity,
{
    debug: DebugContext,
    phantom: PhantomData<E>,
}

impl<E> ELoadBuilder<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new() -> Self {
        Self {
            debug: DebugContext::default(),
            phantom: PhantomData,
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // method
    #[must_use]
    pub const fn method(self, method: LoadMethod) -> ELoadQuery<E> {
        ELoadQuery::new(self, method)
    }

    // all
    #[must_use]
    pub const fn all(self) -> ELoadQuery<E> {
        ELoadQuery::new(self, LoadMethod::All)
    }

    // only
    #[must_use]
    pub const fn only(self) -> ELoadQuery<E> {
        ELoadQuery::new(self, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> ELoadQuery<E> {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        ELoadQuery::new(self, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> ELoadQuery<E> {
        let method = LoadMethod::Many(cks.to_vec());

        ELoadQuery::new(self, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> ELoadQuery<E> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        ELoadQuery::new(self, method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> ELoadQuery<E> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        ELoadQuery::new(self, method)
    }
}

///
/// ELoadQuery
///

#[expect(dead_code)]
pub struct ELoadQuery<E>
where
    E: Entity + 'static,
{
    debug: DebugContext,
    method: LoadMethod,
    offset: u32,
    limit: Option<u32>,
    filter: Option<Filter>,
    order: Option<Order>,
    phantom: PhantomData<E>,
}

impl<E> ELoadQuery<E>
where
    E: Entity + 'static,
{
    #[must_use]
    pub const fn new(builder: ELoadBuilder<E>, method: LoadMethod) -> Self {
        Self {
            debug: builder.debug,
            method,
            offset: 0,
            limit: None,
            filter: None,
            order: None,
            phantom: PhantomData,
        }
    }

    // offset
    #[must_use]
    pub const fn offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    // limit
    #[must_use]
    pub const fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    // limit_option
    #[must_use]
    pub const fn limit_option(mut self, limit: Option<u32>) -> Self {
        self.limit = limit;
        self
    }

    // filter
    #[must_use]
    pub fn filter<T: Into<Filter>>(mut self, filter: T) -> Self {
        self.filter = Some(filter.into());
        self
    }

    // filter_option
    #[must_use]
    pub fn filter_option(mut self, filter: Option<Filter>) -> Self {
        self.filter = filter.map(Into::into);
        self
    }

    // filter_all
    #[must_use]
    pub fn filter_all(mut self, text: &str) -> Self {
        self.filter = Some(Filter::all(text.to_string()));
        self
    }

    // filter_fields
    #[must_use]
    pub fn filter_fields(mut self, fields: &[(String, String)]) -> Self {
        self.filter = Some(Filter::fields(fields.into()));
        self
    }

    // order
    #[must_use]
    pub fn order<T: Into<Order>>(mut self, order: T) -> Self {
        self.order = Some(order.into());
        self
    }

    // order_option
    #[must_use]
    pub fn order_option<T: Into<Order>>(mut self, order: Option<T>) -> Self {
        self.order = order.map(Into::into);
        self
    }

    // execute
    pub fn execute(self, db: &Db) -> Result<ELoadResult<E>, LoadError> {
        let executor = ELoadExecutor::new(self);

        executor.execute(db)
    }
}

///
/// ELoadExecutor
///

pub struct ELoadExecutor<E>
where
    E: Entity + 'static,
{
    query: ELoadQuery<E>,
    resolver: Resolver,
}

impl<E> ELoadExecutor<E>
where
    E: Entity + 'static,
{
    // new
    #[must_use]
    pub fn new(query: ELoadQuery<E>) -> Self {
        Self {
            query,
            resolver: Resolver::new(&E::path()),
        }
    }

    // execute
    // convert into EntityRows and return a RowIterator
    // also make sure we're deserializing the correct entity path
    pub fn execute(self, db: &Db) -> Result<ELoadResult<E>, LoadError> {
        // loader
        let loader = Loader::new(db, &self.resolver);
        let res = loader.load(&self.query.method)?;

        let filtered = res
            .filter(|row| row.value.path == E::path())
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()?;

        let boxed_iter = Box::new(filtered.into_iter()) as Box<dyn Iterator<Item = EntityRow<E>>>;

        Ok(ELoadResult::new(
            boxed_iter,
            self.query.limit,
            self.query.offset,
            self.query.filter,
            self.query.order,
        ))
    }
}
