use crate::{
    db::StoreLocal,
    query::{
        DebugContext, Resolver,
        load::{Error, LoadMethod, LoadResultDyn, Loader},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadBuilderDyn
///

#[derive(Default)]
pub struct LoadBuilderDyn {
    path: String,
}

impl LoadBuilderDyn {
    // new
    #[must_use]
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    // method
    #[must_use]
    pub fn method(self, method: LoadMethod) -> LoadQueryDyn {
        LoadQueryDyn::new(&self.path, method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryDyn {
        LoadQueryDyn::new(&self.path, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryDyn {
        LoadQueryDyn::new(&self.path, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQueryDyn {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQueryDyn::new(&self.path, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQueryDyn {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQueryDyn::new(&self.path, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> Result<LoadQueryDyn, Error> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        Ok(LoadQueryDyn::new(&self.path, method))
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> Result<LoadQueryDyn, Error> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        Ok(LoadQueryDyn::new(&self.path, method))
    }
}

///
/// LoadQueryDyn
///

#[derive(CandidType, Debug, Default, Serialize, Deserialize)]
pub struct LoadQueryDyn {
    path: String,
    method: LoadMethod,
    offset: u32,
    limit: Option<u32>,
    debug: DebugContext,
}

impl LoadQueryDyn {
    // new
    #[must_use]
    pub fn new(path: &str, method: LoadMethod) -> Self {
        Self {
            path: path.to_string(),
            method,
            ..Default::default()
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

    // execute
    pub fn execute(self, store: StoreLocal) -> Result<LoadResultDyn, Error> {
        let executor = LoadExecutorDyn::new(self);

        executor.execute(store)
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
    pub fn execute(self, store: StoreLocal) -> Result<LoadResultDyn, Error> {
        // loader
        let loader = Loader::new(store, self.resolver);
        let rows = loader.load(&self.query.method)?;

        Ok(LoadResultDyn::new(
            rows,
            self.query.limit,
            self.query.offset,
        ))
    }
}
