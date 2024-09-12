use crate::query::{
    iter::{RowIterator, RowIteratorDynamic},
    types::{EntityRow, Filter, LoadMethod, Order},
    DebugContext, Resolver,
};
use crate::{
    types::{DataKey, DataRow},
    Db,
};
use orm::traits::Entity;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::marker::PhantomData;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("filtering not allowed on dynamic loads"))]
    FilterNotAllowed,

    #[snafu(display("key not found: {key}"))]
    KeyNotFound { key: DataKey },

    #[snafu(display("no results found"))]
    NoResultsFound,

    #[snafu(display("range queries not allowed on composite keys"))]
    RangeNotAllowed,

    #[snafu(transparent)]
    Db { source: crate::db::Error },

    #[snafu(transparent)]
    Orm { source: orm::Error },

    #[snafu(transparent)]
    Resolver { source: super::resolver::Error },
}

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
        self.build_options(method)
    }

    // all
    #[must_use]
    pub const fn all(self) -> LoadBuilderOptions<'a, E> {
        self.build_options(LoadMethod::All)
    }

    // only
    #[must_use]
    pub const fn only(self) -> LoadBuilderOptions<'a, E> {
        self.build_options(LoadMethod::One(Vec::new()))
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadBuilderOptions<'a, E> {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();

        self.build_options(LoadMethod::One(ck_str))
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadBuilderOptions<'a, E> {
        self.build_options(LoadMethod::Many(cks.to_vec()))
    }

    // range
    pub fn range<T: ToString>(
        self,
        start: &[T],
        end: &[T],
    ) -> Result<LoadBuilderOptions<'a, E>, crate::Error> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();

        Ok(self.build_options(LoadMethod::Range(start, end)))
    }

    // prefix
    pub fn prefix<T: ToString>(
        self,
        prefix: &[T],
    ) -> Result<LoadBuilderOptions<'a, E>, crate::Error> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();

        Ok(self.build_options(LoadMethod::Prefix(prefix)))
    }

    // build_options
    const fn build_options(self, method: LoadMethod) -> LoadBuilderOptions<'a, E> {
        LoadBuilderOptions::new(self, method)
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
    debug: DebugContext,
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
            debug: prev.debug,
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
    pub fn execute(self) -> Result<RowIterator<E>, crate::Error> {
        let executor = LoadBuilderExecutor::new(self);
        let iter = executor.execute()?;

        Ok(iter)
    }

    // execute_dyn
    pub fn execute_dyn(self) -> Result<RowIteratorDynamic, crate::Error> {
        let executor = LoadBuilderExecutor::new(self);
        let iter = executor.execute_dyn()?;

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
    debug: DebugContext,
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
            debug: prev.debug,
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
    pub fn execute(self) -> Result<RowIterator<E>, Error> {
        let iter = self
            .do_execute()?
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()?;

        let boxed_iter = Box::new(iter.into_iter()) as Box<dyn Iterator<Item = EntityRow<E>>>;

        Ok(RowIterator::new(
            boxed_iter,
            self.limit,
            self.offset,
            self.filter,
            self.order,
        ))
    }

    // execute_dyn
    // cannot currently use filter here
    pub fn execute_dyn(self) -> Result<RowIteratorDynamic, Error> {
        if self.filter.is_some() {
            Err(Error::FilterNotAllowed)?;
        }
        let iter = self.do_execute()?;

        Ok(RowIteratorDynamic::new(iter, self.limit, self.offset))
    }

    // do_execute
    fn do_execute(&self) -> Result<Box<dyn Iterator<Item = DataRow>>, Error> {
        let rows = match &self.method {
            LoadMethod::All => self
                .load_prefix(&Vec::new())
                .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = DataRow>>),

            LoadMethod::One(ck) => self
                .load_one(ck)
                .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = DataRow>>),

            LoadMethod::Many(cks) => self
                .load_many(cks)
                .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = DataRow>>),

            LoadMethod::Prefix(ck) => self
                .load_prefix(ck)
                .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = DataRow>>),

            LoadMethod::Range(start, end) => self
                .load_range(start, end)
                .map(|iter| Box::new(iter) as Box<dyn Iterator<Item = DataRow>>),
        }?;

        Ok(rows)
    }

    ///
    /// PRIVATE LOAD METHODS
    ///

    // load_one
    fn load_one(&self, ck: &[String]) -> Result<impl Iterator<Item = DataRow>, Error> {
        let row = self.by_ck(ck)?;

        Ok(std::iter::once(row))
    }

    // load_many
    fn load_many(&self, cks: &[Vec<String>]) -> Result<impl Iterator<Item = DataRow>, Error> {
        let rows: Result<Vec<_>, _> = cks.iter().map(|ck| self.by_ck(ck)).collect();

        rows.map(Vec::into_iter)
    }

    // load_prefix
    fn load_prefix(&self, prefix: &[String]) -> Result<impl Iterator<Item = DataRow>, Error> {
        let start = E::composite_key(prefix)?;
        let start_sk = self.resolver.data_key(&start)?;
        let end_sk = start_sk.create_upper_bound();

        self.by_range(start_sk, end_sk)
    }

    // load_range
    // composite keys not allowed as B-Trees are one dimensional for lookups
    fn load_range(
        &self,
        start: &[String],
        end: &[String],
    ) -> Result<impl Iterator<Item = DataRow>, Error> {
        let start = E::composite_key(start)?;
        if start.len() != 1 {
            Err(Error::RangeNotAllowed)?;
        }
        let start_sk = self.resolver.data_key(&start)?;

        let end = E::composite_key(end)?;
        let end_sk = self.resolver.data_key(&end)?;

        // create iter over entire alphabetical range
        let iter = self.by_range(start_sk, end_sk)?;

        Ok(iter)
    }

    ///
    /// HELPERS
    ///

    // by_range
    fn by_range(
        &self,
        start: DataKey,
        end: DataKey,
    ) -> Result<impl Iterator<Item = DataRow>, Error> {
        self.debug
            .println(&format!("store.range: {start} -> {end}"));

        // iterate range
        let mut results = Vec::new();
        let store_path = self.resolver.store()?;
        self.db.with_store(&store_path, |store| {
            for (key, value) in store.data.range(start..=end) {
                results.push(DataRow { key, value });
            }

            Ok(())
        })?;

        Ok(results.into_iter())
    }

    // by_ck
    fn by_ck(&self, ck: &[String]) -> Result<DataRow, Error> {
        let key = self.resolver.data_key(ck)?;

        self.by_key(key)
    }

    // by_key
    fn by_key(&self, key: DataKey) -> Result<DataRow, Error> {
        let store_path = &self.resolver.store()?;
        let value = self
            .db
            .with_store(store_path, |store| Ok(store.data.get(&key)))?
            .ok_or_else(|| Error::KeyNotFound { key: key.clone() })?;

        Ok(DataRow { key, value })
    }
}
