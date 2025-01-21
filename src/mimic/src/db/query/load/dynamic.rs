use crate::db::{
    query::{
        iter::RowIteratorDyn,
        load::{Error as LoadError, Loader},
        types::LoadMethod,
        DebugContext, Error as QueryError, Resolver,
    },
    types::DataRow,
    Db,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadBuilderDyn
///

#[derive(Default)]
pub struct LoadBuilderDyn {
    path: String,
    debug: DebugContext,
}

impl LoadBuilderDyn {
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
    pub fn method(self, method: LoadMethod) -> LoadQueryDyn {
        LoadQueryDyn::new(self, method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryDyn {
        LoadQueryDyn::new(self, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryDyn {
        LoadQueryDyn::new(self, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQueryDyn {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQueryDyn::new(self, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQueryDyn {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQueryDyn::new(self, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> Result<LoadQueryDyn, LoadError> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        Ok(LoadQueryDyn::new(self, method))
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> Result<LoadQueryDyn, LoadError> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        Ok(LoadQueryDyn::new(self, method))
    }
}

///
/// LoadQueryDyn
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct LoadQueryDyn {
    path: String,
    debug: DebugContext,
    method: LoadMethod,
    offset: u32,
    limit: Option<u32>,
}

impl LoadQueryDyn {
    #[must_use]
    pub fn new(builder: LoadBuilderDyn, method: LoadMethod) -> Self {
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
    pub fn execute(self, db: &Db) -> Result<RowIteratorDyn, QueryError> {
        let executor = LoadExecutorDyn::new(self);

        executor.execute(db)
    }
}

///
/// LoadExecutorDyn
///

pub struct LoadExecutorDyn {
    query: LoadQueryDyn,
    resolver: Resolver,
}

impl LoadExecutorDyn {
    // new
    #[must_use]
    pub fn new(query: LoadQueryDyn) -> Self {
        let resolver = Resolver::new(&query.path);

        Self { query, resolver }
    }

    // execute
    pub fn execute(self, db: &Db) -> Result<RowIteratorDyn, QueryError> {
        // loader
        let loader = Loader::new(db, &self.resolver);
        let res = loader.load(&self.query.method)?;

        let boxed_iter = Box::new(res.into_iter()) as Box<dyn Iterator<Item = DataRow>>;

        Ok(RowIteratorDyn::new(
            boxed_iter,
            self.query.limit,
            self.query.offset,
        ))
    }
}
