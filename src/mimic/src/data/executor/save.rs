use crate::{
    Error,
    data::{
        DataError,
        executor::{DebugContext, ExecutorError, with_resolver},
        query::{SaveMode, SaveQueryPrepared},
        response::SaveResponse,
        store::{DataStoreRegistry, DataValue, IndexStoreRegistry, Metadata},
    },
    utils::time,
};

///
/// SaveExecutor
///

pub struct SaveExecutor {
    data: DataStoreRegistry,
    indexes: IndexStoreRegistry,
    debug: DebugContext,
}

impl SaveExecutor {
    // new
    #[must_use]
    pub fn new(data: DataStoreRegistry, indexes: IndexStoreRegistry) -> Self {
        Self {
            data,
            indexes,
            debug: DebugContext::default(),
        }
    }

    // debug
    #[must_use]
    pub const fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // execute
    pub fn execute(&self, query: SaveQueryPrepared) -> Result<SaveResponse, Error> {
        let res = self.execute_internal(query)?;

        Ok(res)
    }

    // execute_internal
    fn execute_internal(&self, query: SaveQueryPrepared) -> Result<SaveResponse, DataError> {
        let mode = query.mode;
        let entity = &*query.entity;

        // resolver
        let key_values = &entity.key_values();
        let resolved = with_resolver(|r| r.entity(&entity.path_dyn()))?;
        let sk = resolved.sort_key(key_values);

        // debug
        self.debug.println(&format!("query.{mode}: {sk}"));

        // validate
        let adapter = crate::visit::EntityAdapter(entity);
        crate::validate(&adapter)?;

        // serialize
        let data: Vec<u8> = entity.serialize()?;

        // get old result
        let store = self
            .data
            .with(|data| data.try_get_store(resolved.store_path()))?;
        let result = store.with_borrow(|store| store.get(&sk));

        //
        // match mode
        // on Update and Replace compare old and new data
        //

        let now = time::now_secs();
        let (created, modified) = match mode {
            SaveMode::Create => {
                #[allow(clippy::redundant_clone)]
                if result.is_some() {
                    Err(ExecutorError::KeyExists(sk.clone()))?;
                }

                (now, now)
            }

            SaveMode::Update => match result {
                Some(old) => {
                    let modified = if data == old.data {
                        old.metadata.modified
                    } else {
                        now
                    };

                    (old.metadata.created, modified)
                }
                None => Err(ExecutorError::KeyNotFound(sk.clone()))?,
            },

            SaveMode::Replace => match result {
                Some(old) => {
                    let modified = if data == old.data {
                        old.metadata.modified
                    } else {
                        now
                    };

                    (old.metadata.created, modified)
                }
                None => (now, now),
            },
        };

        // indexes
        for index in resolved.indexes() {
            // Try to build index key from key_values (handles missing/null gracefully)
            let Some(index_key) = resolved.build_index_key(index, key_values) else {
                // Optionally log debug skip reason
                self.debug.println(&format!(
                    "query.{mode}: skipping index {:?} due to missing/null field",
                    index.fields
                ));
                continue;
            };

            let index_store = self.indexes.with(|map| map.try_get_store(&index.store))?;

            index_store.with_borrow_mut(|store| {
                if index.unique {
                    if let Some(existing) = store.data.get(&index_key) {
                        if existing != sk.to_string() {
                            return Err(ExecutorError::IndexViolation(index_key.clone()));
                        }
                    }
                }

                // save id
                if let Some(id) = resolved.id(key_values) {
                    self.debug
                        .println(&format!("query.{mode}: add index {index_key} - {id}"));
                    store.data.insert(index_key, id);
                }

                Ok(())
            })?;
        }

        // prepare data value
        let path = entity.path_dyn();
        let value = DataValue {
            data,
            path,
            metadata: Metadata { created, modified },
        };

        // insert data row
        store.with_borrow_mut(|store| {
            store.data.insert(sk.clone(), value);
        });

        Ok(SaveResponse {
            key: sk,
            created,
            modified,
        })
    }
}
