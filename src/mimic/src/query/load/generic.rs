use crate::{
    Error,
    db::{DbLocal, types::EntityRow},
    orm::traits::Entity,
    query::{
        DebugContext, QueryError, Resolver,
        load::{LoadError, LoadMethod, LoadResponse, Loader},
        types::{Filter, Order},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

///
/// LoadBuilder
///

#[derive(Debug, Default)]
pub struct LoadBuilder<E>
where
    E: Entity,
{
    phantom: PhantomData<E>,
}

impl<E> LoadBuilder<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // method
    #[must_use]
    pub fn method(self, method: LoadMethod) -> LoadQuery {
        LoadQuery::new(E::PATH, method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQuery {
        LoadQuery::new(E::PATH, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQuery {
        LoadQuery::new(E::PATH, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQuery {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQuery::new(E::PATH, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQuery {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQuery::new(E::PATH, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQuery {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        LoadQuery::new(E::PATH, method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQuery {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        LoadQuery::new(E::PATH, method)
    }
}

///
/// LoadQuery
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct LoadQuery {
    pub path: String,
    pub method: LoadMethod,
    pub offset: u32,
    pub limit: Option<u32>,
    pub filter: Option<Filter>,
    pub order: Option<Order>,
    pub debug: DebugContext,
}

impl LoadQuery {
    // new
    #[must_use]
    pub fn new(path: &str, method: LoadMethod) -> Self {
        Self {
            path: path.to_string(),
            method,
            offset: 0,
            limit: None,
            filter: None,
            order: None,
            debug: DebugContext::default(),
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
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
        self.filter = filter;
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
    pub fn execute<E>(self, db: DbLocal) -> Result<LoadResponse<E>, Error>
    where
        E: Entity,
    {
        let executor = LoadExecutor::new(self);

        executor.execute(db)
    }
}

///
/// LoadExecutor
///

pub struct LoadExecutor {
    query: LoadQuery,
}

impl LoadExecutor {
    // new
    #[must_use]
    pub fn new(query: LoadQuery) -> Self {
        Self { query }
    }

    // execute
    // also make sure we're deserializing the correct entity path
    pub fn execute<E>(self, db: DbLocal) -> Result<LoadResponse<E>, Error>
    where
        E: Entity,
    {
        // loader
        let resolver = Resolver::new(&self.query.path);
        let loader = Loader::new(db, resolver);
        let res = loader.load(&self.query.method)?;

        let rows = res
            .into_iter()
            .filter(|row| row.value.path == E::path())
            .map(TryFrom::try_from)
            .collect::<Result<Vec<EntityRow<E>>, _>>()
            .map_err(LoadError::SerializeError)
            .map_err(QueryError::LoadError)?;

        Ok(LoadResponse::new(
            rows,
            self.query.limit,
            self.query.offset,
            self.query.filter,
            self.query.order,
        ))
    }
}
