pub mod response;

pub use response::{LoadResponse, LoadResponseDyn};

use crate::{
    Error, ThisError,
    db::{
        DbError, DbLocal, StoreLocal,
        types::{DataKey, DataRow, EntityRow},
    },
    ic::serialize::SerializeError,
    orm::traits::Entity,
    query::{
        DebugContext, QueryError,
        resolver::{Resolver, ResolverError},
        types::{Filter, Order},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum LoadError {
    #[error("method can only be set once")]
    MethodAlreadySet,

    #[error("method is not set")]
    MethodNotSet,

    #[error("key not found: {0}")]
    KeyNotFound(DataKey),

    #[error("no results found")]
    NoResultsFound,

    #[error("range queries not allowed on composite keys")]
    RangeNotAllowed,

    #[error(transparent)]
    DbError(#[from] DbError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    ResolverError(#[from] ResolverError),
}

///
/// LoadFormat
///
/// a variant that specifies the format the LoadResponse should be in
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum LoadFormat {
    Rows,
    Count,
}

///
/// LoadMethod
///
/// All    : no sort key prefix, only works with top-level Sort Keys,
///          will probably not work if used on nested entities
/// Only   : for entities that have no keys
/// One    : returns one row by composite key
/// Many   : returns many rows (from many composite keys)
/// Prefix : like all but we're asking for the composite key prefix
///          so Pet (Character=1) will return the Pets from Character 1
/// Range  : user-defined range, ie. Item=1000 Item=1500
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub enum LoadMethod {
    All,
    Only,
    One(Vec<String>),
    Many(Vec<Vec<String>>),
    Prefix(Vec<String>),
    Range(Vec<String>, Vec<String>),
}

///
/// LoadBuilder
///

#[derive(Debug)]
pub struct LoadBuilder {
    path: String,
}

impl LoadBuilder {
    // new
    #[must_use]
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    // method
    #[must_use]
    pub fn method(self, method: LoadMethod) -> LoadQuery {
        LoadQuery::new(&self.path, method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQuery {
        LoadQuery::new(&self.path, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQuery {
        LoadQuery::new(&self.path, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQuery {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQuery::new(&self.path, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQuery {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQuery::new(&self.path, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQuery {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        LoadQuery::new(&self.path, method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQuery {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        LoadQuery::new(&self.path, method)
    }
}

///
/// LoadQuery
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
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
    pub fn execute(self, db: DbLocal) -> Result<LoadResponseDyn, Error> {
        let executor = LoadExecutor::new(db, self);

        executor.execute()
    }

    // execute_as
    pub fn execute_as<E: Entity>(self, db: DbLocal) -> Result<LoadResponse<E>, Error> {
        let executor = LoadExecutor::new(db, self);

        executor.execute_as()
    }
}

///
/// LoadExecutor
///

pub struct LoadExecutor {
    query: LoadQuery,
    db: DbLocal,
    resolver: Resolver,
}

impl LoadExecutor {
    // new
    #[must_use]
    pub fn new(db: DbLocal, query: LoadQuery) -> Self {
        let resolver = Resolver::new(&query.path);

        Self {
            db,
            query,
            resolver,
        }
    }

    // execute
    pub fn execute(self) -> Result<LoadResponseDyn, Error> {
        let data_rows = self.load(&self.query.method)?;

        Ok(LoadResponseDyn::new(
            data_rows,
            self.query.limit,
            self.query.offset,
        ))
    }

    // execute_as
    // also make sure we're deserializing the correct entity path
    pub fn execute_as<E: Entity>(self) -> Result<LoadResponse<E>, Error> {
        let data_rows = self.load(&self.query.method)?;

        let rows = data_rows
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

    // load
    pub fn load(&self, method: &LoadMethod) -> Result<Vec<DataRow>, Error> {
        self.load_unmapped(method)
            .map_err(QueryError::LoadError)
            .map_err(Error::QueryError)
    }

    // load_unmapped
    // for easier error wrapping
    fn load_unmapped(&self, method: &LoadMethod) -> Result<Vec<DataRow>, LoadError> {
        let store_path = &self.resolver.store()?;
        let store = self.db.with(|db| db.try_get_store(store_path))?;

        let res = match method {
            LoadMethod::All | LoadMethod::Only => {
                let start = self.data_key(&[])?;
                let end = start.create_upper_bound();

                query_range(store, start, end)
            }

            LoadMethod::One(ck) => {
                let key = self.data_key(ck)?;
                let row = query_data_key(store, key)?;

                vec![row]
            }

            LoadMethod::Many(cks) => {
                let keys = cks
                    .iter()
                    .map(|ck| self.data_key(ck))
                    .collect::<Result<Vec<_>, _>>()?;

                keys.into_iter()
                    .map(|key| query_data_key(store, key))
                    .filter_map(Result::ok)
                    .collect::<Vec<_>>()
            }

            LoadMethod::Prefix(prefix) => {
                let start = self.data_key(prefix)?;
                let end = start.create_upper_bound();

                query_range(store, start, end)
            }

            LoadMethod::Range(start_ck, end_ck) => {
                let start = self.data_key(start_ck)?;
                let end = self.data_key(end_ck)?;

                query_range(store, start, end)
            }
        };

        Ok(res)
    }

    // data_key
    // for easy error converstion
    fn data_key(&self, ck: &[String]) -> Result<DataKey, LoadError> {
        let key = self.resolver.data_key(ck)?;

        Ok(key)
    }
}

// query_data_key
fn query_data_key(store: StoreLocal, key: DataKey) -> Result<DataRow, LoadError> {
    store.with_borrow(|this| {
        this.get(&key)
            .map(|value| DataRow {
                key: key.clone(),
                value,
            })
            .ok_or_else(|| LoadError::KeyNotFound(key.clone()))
    })
}

// query_range
fn query_range(store: StoreLocal, start: DataKey, end: DataKey) -> Vec<DataRow> {
    store.with_borrow(|this| {
        this.range(start..=end)
            .map(|(key, value)| DataRow { key, value })
            .collect()
    })
}
