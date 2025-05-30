use crate::{
    Error,
    db::DbLocal,
    query::{
        DebugContext, QueryError, Resolver,
        delete::{DeleteError, DeleteMethod, DeleteQueryDyn, DeleteResponse},
    },
};

///
/// DeleteQueryDynInit
///

#[derive(Debug, Default)]
pub struct DeleteQueryDynInit {}

impl DeleteQueryDynInit {
    // new
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    // query
    #[must_use]
    pub fn query(self, query: DeleteQueryDyn) -> DeleteQueryDynBuilder {
        DeleteQueryDynBuilder::new(query)
    }

    // one
    pub fn one<S: ToString>(self, path: &str, ck: &[S]) -> DeleteQueryDynBuilder {
        let key = ck.iter().map(ToString::to_string).collect();
        let method = DeleteMethod::One(key);

        DeleteQueryDynBuilder::new_with(path, method)
    }

    // many
    #[must_use]
    pub fn many<S: ToString>(self, path: &str, ck: &[Vec<S>]) -> DeleteQueryDynBuilder {
        let keys: Vec<Vec<String>> = ck
            .iter()
            .map(|inner_vec| inner_vec.iter().map(ToString::to_string).collect())
            .collect();
        let method = DeleteMethod::Many(keys);

        DeleteQueryDynBuilder::new_with(path, method)
    }
}

///
/// DeleteQueryDynBuilder
///

#[derive(Default)]
pub struct DeleteQueryDynBuilder {
    query: DeleteQueryDyn,
    debug: DebugContext,
}

impl DeleteQueryDynBuilder {
    // new
    #[must_use]
    pub fn new(query: DeleteQueryDyn) -> Self {
        Self {
            query,
            ..Default::default()
        }
    }

    // new_with
    #[must_use]
    pub fn new_with(path: &str, method: DeleteMethod) -> Self {
        let query = DeleteQueryDyn::new(path, method);

        Self {
            query,
            ..Default::default()
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // execute
    pub fn execute(self, db: DbLocal) -> Result<DeleteResponse, Error> {
        let executor = DeleteQueryDynExecutor::new(self);
        executor.execute(db)
    }
}

///
/// DeleteQueryExecutor
///

pub struct DeleteQueryDynExecutor {
    builder: DeleteQueryDynBuilder,
    resolver: Resolver,
}

impl DeleteQueryDynExecutor {
    // new
    #[must_use]
    pub fn new(builder: DeleteQueryDynBuilder) -> Self {
        let resolver = Resolver::new(&builder.query.path);

        Self { builder, resolver }
    }

    // execute
    pub fn execute(&self, db: DbLocal) -> Result<DeleteResponse, Error> {
        let query = &self.builder.query;

        let keys = match &query.method {
            DeleteMethod::Undefined => {
                return Err(QueryError::DeleteError(DeleteError::Undefined))?;
            }
            DeleteMethod::One(key) => vec![key],
            DeleteMethod::Many(keys) => keys.iter().collect(),
        };

        // debug
        self.builder
            .debug
            .println(&format!("delete: keys {keys:?}"));

        // get store
        let store_path = &self.resolver.store().map_err(QueryError::ResolverError)?;
        let store = db
            .with(|db| db.try_get_store(store_path))
            .map_err(QueryError::DbError)?;

        // execute for every different key
        let mut deleted_keys = Vec::new();
        for key in keys {
            let data_key = self
                .resolver
                .data_key(key)
                .map_err(QueryError::ResolverError)?;

            // remove returns DataValue but we ignore it for now
            // if the key is deleted then add it to the vec
            if store
                .with_borrow_mut(|store| store.remove(&data_key))
                .is_some()
            {
                deleted_keys.push(data_key);
            }
        }

        // debug
        self.builder
            .debug
            .println(&format!("keys deleted: {deleted_keys:?}"));

        Ok(DeleteResponse(deleted_keys))
    }
}
