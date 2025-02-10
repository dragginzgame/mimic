use crate::{
    query::{
        load::{Error, LoadResultDyn, Loader},
        types::LoadMethod,
        DebugContext, Resolver,
    },
    store::{types::DataRow, StoreLocal},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// LoadBuilderPath
///

#[derive(Default)]
pub struct LoadBuilderPath {
    debug: DebugContext,
}

impl LoadBuilderPath {
    // new
    #[must_use]
    pub fn new() -> Self {
        Self {
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
    pub fn method(self, method: LoadMethod) -> LoadQueryPath {
        LoadQueryPath::from_builder(self, method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryPath {
        LoadQueryPath::from_builder(self, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryPath {
        LoadQueryPath::from_builder(self, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQueryPath {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQueryPath::from_builder(self, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQueryPath {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQueryPath::from_builder(self, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> Result<LoadQueryPath, Error> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        Ok(LoadQueryPath::from_builder(self, method))
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> Result<LoadQueryPath, Error> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        Ok(LoadQueryPath::from_builder(self, method))
    }
}

///
/// LoadQueryPath
///

#[derive(CandidType, Debug, Default, Serialize, Deserialize)]
pub struct LoadQueryPath {
    path: String,
    debug: DebugContext,
    method: LoadMethod,
    offset: u32,
    limit: Option<u32>,
}

impl LoadQueryPath {
    #[must_use]
    pub fn new(path: &str, method: LoadMethod) -> Self {
        Self {
            path: path.to_string(),
            method,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn from_builder(builder: LoadBuilderPath, method: LoadMethod) -> Self {
        Self {
            debug: builder.debug,
            method,
            ..Default::default()
        }
    }

    // path
    #[must_use]
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
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
        let executor = LoadExecutorPath::new(self);

        executor.execute(store)
    }
}

///
/// LoadExecutorPath
///

pub struct LoadExecutorPath {
    query: LoadQueryPath,
    resolver: Resolver,
}

impl LoadExecutorPath {
    // new
    #[must_use]
    pub fn new(query: LoadQueryPath) -> Self {
        let resolver = Resolver::new(&query.path);

        Self { query, resolver }
    }

    // execute
    pub fn execute(self, store: StoreLocal) -> Result<LoadResultDyn, Error> {
        // loader
        let loader = Loader::new(store, &self.resolver);
        let res = loader.load(&self.query.method)?;

        let boxed_iter = Box::new(res.into_iter()) as Box<dyn Iterator<Item = DataRow>>;

        Ok(LoadResultDyn::new(
            boxed_iter,
            self.query.limit,
            self.query.offset,
        ))
    }
}
