use crate::{
    Error,
    core::{Key, Value, deserialize, traits::EntityKind},
    db::{
        DbError,
        executor::Context,
        query::{DeleteQuery, FilterBuilder, QueryValidate},
        store::{DataStoreRegistryLocal, IndexStoreRegistryLocal},
    },
    debug,
};

///
/// DeleteExecutor
///

#[derive(Clone, Copy, Debug)]
pub struct DeleteExecutor {
    data_registry: DataStoreRegistryLocal,
    index_registry: IndexStoreRegistryLocal,
    debug: bool,
}

impl DeleteExecutor {
    // new
    #[must_use]
    pub const fn new(
        data_registry: DataStoreRegistryLocal,
        index_registry: IndexStoreRegistryLocal,
    ) -> Self {
        Self {
            data_registry,
            index_registry,
            debug: false,
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    ///
    /// HELPER METHODS
    /// these will create an intermediate query
    ///

    pub fn one<E: EntityKind>(&self, value: impl Into<Value>) -> Result<Vec<Key>, Error> {
        self.execute::<E>(DeleteQuery::new().one::<E>(value))
    }

    pub fn many<E: EntityKind>(
        &self,
        values: impl IntoIterator<Item = impl Into<Value>>,
    ) -> Result<Vec<Key>, Error> {
        self.execute::<E>(DeleteQuery::new().many::<E>(values))
    }

    pub fn all<E: EntityKind>(&self) -> Result<Vec<Key>, Error> {
        self.execute::<E>(DeleteQuery::new())
    }

    pub fn filter<E: EntityKind>(
        self,
        f: impl FnOnce(FilterBuilder) -> FilterBuilder,
    ) -> Result<Vec<Key>, Error> {
        self.execute::<E>(DeleteQuery::new().filter(f))
    }

    ///
    /// EXECUTION METHODS
    ///

    fn context(&self) -> Context {
        Context {
            data_registry: self.data_registry,
            index_registry: self.index_registry,
            debug: self.debug,
        }
    }

    // response
    // for the automated query endpoint, we will make this more flexible in the future
    pub fn response<E: EntityKind>(self, query: DeleteQuery) -> Result<Vec<Key>, Error> {
        let res = self.execute::<E>(query)?;

        Ok(res)
    }

    // execute
    pub fn execute<E: EntityKind>(self, query: DeleteQuery) -> Result<Vec<Key>, Error> {
        QueryValidate::<E>::validate(&query).map_err(DbError::from)?;

        let ctx = self.context();

        // plan + stores
        let plan = ctx.plan::<E>(query.filter.as_ref());
        let store = ctx.store::<E>()?;

        // candidates from plan
        let mut keys = ctx.candidates_from_plan::<E>(plan)?;

        // filter (key-level) BEFORE limit
        if let Some(f) = &query.filter {
            keys = ctx.filter_keys::<E>(&store, keys, f);
        }

        // apply limit AFTER filtering
        if let Some(lim) = query.limit.as_ref().and_then(|l| l.limit) {
            keys.truncate(lim as usize);
        }

        // single mutable borrow to delete + drop indexes
        let mut deleted_rows = Vec::new();
        store.with_borrow_mut(|s| {
            for dk in keys {
                if let Some(data_value) = s.get(&dk) {
                    s.remove(&dk);

                    if !E::INDEXES.is_empty() {
                        let entity: E = deserialize(&data_value.bytes)?;
                        self.remove_indexes::<E>(&entity)?;
                    }

                    deleted_rows.push(dk.key());
                }
            }
            Ok::<_, DbError>(())
        })?;

        debug!(self.debug, "query.delete: deleted keys {deleted_rows:?}");

        Ok(deleted_rows)
    }

    // remove_indexes
    fn remove_indexes<E: EntityKind>(&self, entity: &E) -> Result<(), DbError> {
        for index in E::INDEXES {
            let store = self
                .index_registry
                .with(|reg| reg.try_get_store(index.store))?;

            store.with_borrow_mut(|this| {
                this.remove_index_entry(entity, index);
            });
        }

        Ok(())
    }
}
