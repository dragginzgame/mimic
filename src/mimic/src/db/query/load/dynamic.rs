use crate::db::{
    query::{
        iter::RowIteratorDyn,
        load::{Error as LoadError, Loader},
        types::LoadMethod,
        DebugContext, Resolver,
    },
    types::DataRow,
    Db,
};

///
/// LoadBuilderDyn
///

pub struct LoadBuilderDyn<'a> {
    db: &'a Db,
    path: String,
    debug: DebugContext,
}

impl<'a> LoadBuilderDyn<'a> {
    // new
    #[must_use]
    pub fn new(db: &'a Db, path: String) -> Self {
        Self {
            db,
            path,
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
    pub fn method(self, method: LoadMethod) -> LoadBuilderDynOptions<'a> {
        LoadBuilderDynOptions::new(self, method)
    }

    // all
    #[must_use]
    pub fn all(self) -> LoadBuilderDynOptions<'a> {
        LoadBuilderDynOptions::new(self, LoadMethod::All)
    }

    // only
    #[must_use]
    pub fn only(self) -> LoadBuilderDynOptions<'a> {
        LoadBuilderDynOptions::new(self, LoadMethod::Only)
    }

    // one
    pub fn one<T: ToString>(self, ck: &[T]) -> LoadBuilderDynOptions<'a> {
        let ck_str: Vec<String> = ck.iter().map(ToString::to_string).collect();
        let method = LoadMethod::One(ck_str);

        LoadBuilderDynOptions::new(self, method)
    }

    // many
    #[must_use]
    pub fn many(self, cks: &[Vec<String>]) -> LoadBuilderDynOptions<'a> {
        let method = LoadMethod::Many(cks.to_vec());

        LoadBuilderDynOptions::new(self, method)
    }

    // range
    pub fn range<T: ToString>(
        self,
        start: &[T],
        end: &[T],
    ) -> Result<LoadBuilderDynOptions<'a>, LoadError> {
        let start = start.iter().map(ToString::to_string).collect();
        let end = end.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Range(start, end);

        Ok(LoadBuilderDynOptions::new(self, method))
    }

    // prefix
    pub fn prefix<T: ToString>(self, prefix: &[T]) -> Result<LoadBuilderDynOptions<'a>, LoadError> {
        let prefix: Vec<String> = prefix.iter().map(ToString::to_string).collect();
        let method = LoadMethod::Prefix(prefix);

        Ok(LoadBuilderDynOptions::new(self, method))
    }
}

///
/// LoadBuilderDynOptions
///

pub struct LoadBuilderDynOptions<'a> {
    db: &'a Db,
    path: String,
    //   debug: DebugContext,
    method: LoadMethod,
    offset: u32,
    limit: Option<u32>,
}

impl<'a> LoadBuilderDynOptions<'a> {
    #[must_use]
    pub fn new(prev: LoadBuilderDyn<'a>, method: LoadMethod) -> Self {
        Self {
            db: prev.db,
            path: prev.path,
            //          debug: prev.debug,
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
    pub fn execute(self) -> Result<RowIteratorDyn, LoadError> {
        let executor = LoadBuilderDynExecutor::new(self);
        let iter = executor.execute()?;

        Ok(iter)
    }
}

///
/// LoadBuilderDynExecutor
///

pub struct LoadBuilderDynExecutor<'a> {
    db: &'a Db,
    //  debug: DebugContext,
    method: LoadMethod,
    limit: Option<u32>,
    offset: u32,
    resolver: Resolver,
}

impl<'a> LoadBuilderDynExecutor<'a> {
    // new
    #[must_use]
    pub fn new(prev: LoadBuilderDynOptions<'a>) -> Self {
        Self {
            db: prev.db,
            //        debug: prev.debug,
            method: prev.method,
            limit: prev.limit,
            offset: prev.offset,
            resolver: Resolver::new(&prev.path),
        }
    }

    // execute
    pub fn execute(self) -> Result<RowIteratorDyn, LoadError> {
        // loader
        let loader = Loader::new(self.db, &self.resolver);
        let res = loader.load(&self.method)?;

        let boxed_iter = Box::new(res.into_iter()) as Box<dyn Iterator<Item = DataRow>>;

        Ok(RowIteratorDyn::new(boxed_iter, self.limit, self.offset))
    }
}
