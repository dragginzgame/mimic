use crate::{
    db::{
        query::{
            iter::RowIterator,
            load::{Error as LoadError, Loader},
            types::{EntityRow, Filter, LoadMethod, Order},
            DebugContext, Error as QueryError, Resolver,
        },
        Db,
    },
    orm::traits::Entity,
};
use std::marker::PhantomData;

///
/// LoadBuilder
///

pub struct LoadBuilder<'a, E>
where
    E: Entity + 'static,
{
    db: &'a Db,
    debug: DebugContext,
    phantom: PhantomData<E>,
}

impl<'a, E> LoadBuilder<'a, E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new(db: &'a Db) -> Self {
        Self {
            db,
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
    pub const fn method(self, method: LoadMethod) -> LoadBuilderOptions<'a, E> {
        LoadBuilderOptions::new(self, method)
    }

    // all
    #[must_use]
    pub const fn all(self) -> LoadBuilderOptions<'a, E> {
        LoadBuilderOptions::new(self, LoadMethod::All)
    }

    // only
    #[must_use]
    pub const fn only(self) -> LoadBuilderOptions<'a, E> {
        LoadBuilderOptions::new(self, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadBuilderOptions<'a, E> {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadBuilderOptions::new(self, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadBuilderOptions<'a, E> {
        let method = LoadMethod::Many(cks.to_vec());

        LoadBuilderOptions::new(self, method)
    }

    // range
    pub fn range<T: ToString>(
        self,
        start: &[T],
        end: &[T],
    ) -> Result<LoadBuilderOptions<'a, E>, LoadError> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        Ok(LoadBuilderOptions::new(self, method))
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> Result<LoadBuilderOptions<'a, E>, LoadError> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        Ok(LoadBuilderOptions::new(self, method))
    }
}

///
/// LoadBuilderOptions
///

pub struct LoadBuilderOptions<'a, E>
where
    E: Entity,
{
    db: &'a Db,
    //    debug: DebugContext,
    method: LoadMethod,
    offset: u32,
    limit: Option<u32>,
    filter: Option<Filter>,
    order: Option<Order>,
    phantom: PhantomData<E>,
}

impl<'a, E> LoadBuilderOptions<'a, E>
where
    E: Entity + 'static,
{
    #[must_use]
    pub const fn new(prev: LoadBuilder<'a, E>, method: LoadMethod) -> Self {
        Self {
            db: prev.db,
            //     debug: prev.debug,
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
    pub fn execute(self) -> Result<RowIterator<E>, QueryError> {
        let executor = LoadBuilderExecutor::new(self);
        let iter = executor.execute()?;

        Ok(iter)
    }
}

///
/// LoadBuilderExecutor
///

pub struct LoadBuilderExecutor<'a, E>
where
    E: Entity,
{
    db: &'a Db,
    //   debug: DebugContext,
    method: LoadMethod,
    limit: Option<u32>,
    offset: u32,
    filter: Option<Filter>,
    order: Option<Order>,
    resolver: Resolver,
    phantom: PhantomData<E>,
}

impl<'a, E> LoadBuilderExecutor<'a, E>
where
    E: Entity + 'static,
{
    // new
    #[must_use]
    pub fn new(prev: LoadBuilderOptions<'a, E>) -> Self {
        Self {
            db: prev.db,
            //       debug: prev.debug,
            method: prev.method,
            limit: prev.limit,
            offset: prev.offset,
            filter: prev.filter,
            order: prev.order,
            resolver: Resolver::new(&E::path()),
            phantom: PhantomData,
        }
    }

    // execute
    // convert into EntityRows and return a RowIterator
    // also make sure we're deserializing the correct entity path
    pub fn execute(self) -> Result<RowIterator<E>, QueryError> {
        // loader
        let loader = Loader::new(self.db, &self.resolver);
        let res = loader.load(&self.method)?;

        let filtered = res
            .filter(|row| row.value.path == E::path())
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()
            .map_err(LoadError::from)?;

        let boxed_iter = Box::new(filtered.into_iter()) as Box<dyn Iterator<Item = EntityRow<E>>>;

        Ok(RowIterator::new(
            boxed_iter,
            self.limit,
            self.offset,
            self.filter,
            self.order,
        ))
    }
}
