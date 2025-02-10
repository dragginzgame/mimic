use crate::{
    orm::traits::Entity,
    query::{
        load::{LoadError, LoadResult, Loader},
        types::{Filter, LoadMethod, Order},
        DebugContext, QueryError, Resolver,
    },
    store::{types::EntityRow, StoreLocal},
    Error,
};
use candid::CandidType;
use serde::Serialize;
use std::marker::PhantomData;

///
/// LoadBuilder
///

#[derive(Default)]
pub struct LoadBuilder<E>
where
    E: Entity,
{
    debug: DebugContext,
    phantom: PhantomData<E>,
}

impl<E> LoadBuilder<E>
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
    pub const fn method(self, method: LoadMethod) -> LoadQuery<E> {
        LoadQuery::from_builder(self, method)
    }

    // all
    #[must_use]
    pub const fn all(self) -> LoadQuery<E> {
        LoadQuery::from_builder(self, LoadMethod::All)
    }

    // only
    #[must_use]
    pub const fn only(self) -> LoadQuery<E> {
        LoadQuery::from_builder(self, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQuery<E> {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQuery::from_builder(self, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQuery<E> {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQuery::from_builder(self, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQuery<E> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        LoadQuery::from_builder(self, method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQuery<E> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        LoadQuery::from_builder(self, method)
    }
}

///
/// LoadQuery
///

#[derive(CandidType, Debug, Default, Serialize)]
pub struct LoadQuery<E>
where
    E: Entity + 'static,
{
    method: LoadMethod,
    offset: u32,
    limit: Option<u32>,
    filter: Option<Filter>,
    order: Option<Order>,
    debug: DebugContext,
    phantom: PhantomData<E>,
}

impl<E> LoadQuery<E>
where
    E: Entity + Default + 'static,
{
    // new
    #[must_use]
    pub fn new(method: LoadMethod) -> Self {
        Self {
            method,
            ..Default::default()
        }
    }

    // from_builder
    #[must_use]
    pub const fn from_builder(builder: LoadBuilder<E>, method: LoadMethod) -> Self {
        Self {
            method,
            offset: 0,
            limit: None,
            filter: None,
            order: None,
            debug: builder.debug,
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
    pub fn execute(self, store: StoreLocal) -> Result<LoadResult<E>, Error> {
        let executor = LoadExecutor::new(self);

        executor.execute(store)
    }
}

///
/// LoadExecutor
///

pub struct LoadExecutor<E>
where
    E: Entity + 'static,
{
    query: LoadQuery<E>,
    resolver: Resolver,
}

impl<E> LoadExecutor<E>
where
    E: Entity + 'static,
{
    // new
    #[must_use]
    pub fn new(query: LoadQuery<E>) -> Self {
        Self {
            query,
            resolver: Resolver::new(&E::path()),
        }
    }

    // execute
    // convert into EntityRows and return a RowIterator
    // also make sure we're deserializing the correct entity path
    pub fn execute(self, store: StoreLocal) -> Result<LoadResult<E>, Error> {
        // loader
        let loader = Loader::new(store, &self.resolver);
        let res = loader.load(&self.query.method)?;

        let filtered = res
            .filter(|row| row.value.path == E::path())
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()
            .map_err(LoadError::SerializeError)
            .map_err(QueryError::LoadError)?;

        let boxed_iter = Box::new(filtered.into_iter()) as Box<dyn Iterator<Item = EntityRow<E>>>;

        Ok(LoadResult::new(
            boxed_iter,
            self.query.limit,
            self.query.offset,
            self.query.filter,
            self.query.order,
        ))
    }
}
