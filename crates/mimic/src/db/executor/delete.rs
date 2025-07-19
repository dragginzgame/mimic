use crate::{
    MimicError,
    core::{Value, traits::EntityKind},
    db::{
        DbError, ExecutorError,
        query::{DeleteQuery, QueryPlan, QueryShape},
        response::{DeleteCollection, DeleteRow},
        store::{DataKey, DataStoreRegistryLocal, IndexKey, IndexStoreRegistryLocal},
    },
    debug,
    serialize::deserialize,
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

    // one
    // helper method, creates query
    pub fn one<E: EntityKind>(
        &self,
        value: impl Into<Value>,
    ) -> Result<DeleteCollection, MimicError> {
        self.execute::<E>(DeleteQuery::new().one::<E>(value))
    }

    // many
    // helper method, creates query
    pub fn many<E, I>(&self, values: I) -> Result<DeleteCollection, MimicError>
    where
        E: EntityKind,
        I: IntoIterator,
        I::Item: Into<Value>,
    {
        self.execute::<E>(DeleteQuery::new().many::<E, I>(values))
    }

    // all
    pub fn all<E: EntityKind>(&self) -> Result<DeleteCollection, MimicError> {
        self.execute::<E>(DeleteQuery::new())
    }

    // execute
    pub fn execute<E: EntityKind>(
        self,
        query: DeleteQuery,
    ) -> Result<DeleteCollection, MimicError> {
        let res = self.execute_internal::<E>(query)?;

        Ok(res)
    }

    // execute_internal
    fn execute_internal<E: EntityKind>(
        &self,
        query: DeleteQuery,
    ) -> Result<DeleteCollection, DbError> {
        let plan = QueryPlan::new(&query.filter);
        let shape = plan.shape::<E>();

        debug!(
            self.debug,
            "query.delete: query is {query:?}, shape is {shape:?}"
        );

        // resolve shape
        let data_keys: Vec<DataKey> = match shape {
            QueryShape::One(key) => vec![key],
            QueryShape::Many(entity_keys) => entity_keys,

            _ => return Err(ExecutorError::ShapeNotSupported)?,
        };

        // get store
        let store = self.data_registry.with(|db| db.get_store::<E>());

        // execute for every different key
        let mut deleted_rows = Vec::new();
        for dk in data_keys {
            let Some(data_value) = store.with_borrow(|s| s.get(&dk)) else {
                continue;
            };

            // deserialize and remove indexes
            let entity: E = deserialize(&data_value.bytes)?;
            self.remove_indexes::<E>(&entity)?;

            // delete
            store.with_borrow_mut(|s| {
                s.remove(&dk);
            });

            deleted_rows.push(DeleteRow::new(dk.key()));
        }

        // debug
        debug!(self.debug, "query.delete: deleted keys {deleted_rows:?}");

        Ok(DeleteCollection(deleted_rows))
    }

    // remove_indexes
    fn remove_indexes<E: EntityKind>(&self, entity: &E) -> Result<(), DbError> {
        /*
                let key = entity.key();

                for index in E::INDEXES {
                    // remove index if found
                    if let Some(index_key) = IndexKey::build(entity, index.fields) {
                        let index_store = self.index_registry.with(|ix| ix.get_store(index.store));

                        index_store.with_borrow_mut(|store| {
                            store.remove_index_entry(&index_key, &key);
                        });
                    }
                }
        */

        Ok(())
    }
}
