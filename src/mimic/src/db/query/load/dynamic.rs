use crate::db::{
    query::{
        load::{LoadError, LoadResult, Loader},
        types::LoadMethod,
        DebugContext, Resolver,
    },
    types::DataRow,
    Db,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadBuilder
///

#[derive(Default)]
pub struct LoadBuilder {
    path: String,
    debug: DebugContext,
}

impl LoadBuilder {
    // new
    #[must_use]
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            debug: DebugContext::default(),
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
    pub fn method(self, method: LoadMethod) -> LoadQuery {
        LoadQuery::new(self, method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQuery {
        LoadQuery::new(self, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQuery {
        LoadQuery::new(self, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQuery {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQuery::new(self, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQuery {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQuery::new(self, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> Result<LoadQuery, LoadError> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        Ok(LoadQuery::new(self, method))
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> Result<LoadQuery, LoadError> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        Ok(LoadQuery::new(self, method))
    }
}

///
/// LoadQuery
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct LoadQuery {
    path: String,
    debug: DebugContext,
    method: LoadMethod,
    offset: u32,
    limit: Option<u32>,
}

impl LoadQuery {
    #[must_use]
    pub fn new(builder: LoadBuilder, method: LoadMethod) -> Self {
        Self {
            path: builder.path,
            debug: builder.debug,
            method,
            offset: 0,
            limit: None,
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

    // execute
    pub fn execute(self, db: &Db) -> Result<LoadResult, LoadError> {
        let executor = LoadExecutor::new(self);

        executor.execute(db)
    }
}

///
/// LoadExecutor
///

pub struct LoadExecutor {
    query: LoadQuery,
    resolver: Resolver,
}

impl LoadExecutor {
    // new
    #[must_use]
    pub fn new(query: LoadQuery) -> Self {
        let resolver = Resolver::new(&query.path);

        Self { query, resolver }
    }

    // execute
    pub fn execute(self, db: &Db) -> Result<LoadResult, LoadError> {
        // loader
        let loader = Loader::new(db, &self.resolver);
        let res = loader.load(&self.query.method)?;
        let boxed_iter = Box::new(res.into_iter()) as Box<dyn Iterator<Item = DataRow>>;

        Ok(LoadResult::new(
            boxed_iter,
            self.query.limit,
            self.query.offset,
        ))
    }
}
