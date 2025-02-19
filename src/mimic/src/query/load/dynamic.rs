use crate::{
    db::DbLocal,
    orm::traits::Entity,
    query::{
        DebugContext, Resolver,
        load::{Error, LoadMethod, LoadResponseDyn, Loader},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

///
/// LoadBuilderDyn
///

#[derive(Debug, Default)]
pub struct LoadBuilderDyn<E>
where
    E: Entity,
{
    phantom: PhantomData<E>,
}

impl<E> LoadBuilderDyn<E>
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
    pub fn method(self, method: LoadMethod) -> LoadQueryDyn {
        LoadQueryDyn::new(E::PATH, method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadQueryDyn {
        LoadQueryDyn::new(E::PATH, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadQueryDyn {
        LoadQueryDyn::new(E::PATH, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadQueryDyn {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadQueryDyn::new(E::PATH, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadQueryDyn {
        let method = LoadMethod::Many(cks.to_vec());

        LoadQueryDyn::new(E::PATH, method)
    }

    // range
    pub fn range<T: ToString>(self, start: &[T], end: &[T]) -> LoadQueryDyn {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        LoadQueryDyn::new(E::PATH, method)
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> LoadQueryDyn {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        LoadQueryDyn::new(E::PATH, method)
    }
}

///
/// LoadQueryDyn
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
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
            offset: 0,
            limit: None,
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

    // execute
    pub fn execute(self, db: DbLocal) -> Result<LoadResponseDyn, Error> {
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
    pub fn execute(self, db: DbLocal) -> Result<LoadResponseDyn, Error> {
        // loader
        let loader = Loader::new(db, self.resolver);
        let rows = loader.load(&self.query.method)?;

        Ok(LoadResponseDyn::new(
            rows,
            self.query.limit,
            self.query.offset,
        ))
    }
}
