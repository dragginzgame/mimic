use crate::{
    Error,
    db::{
        DataStoreRegistry, IndexStoreRegistry,
        types::{DataValue, Metadata, SortKey},
    },
    query::{SaveMode, SaveQueryPrepared, SaveResponse},
    service::{
        ServiceError,
        storage::{DebugContext, ResolverError, StorageError, with_resolver},
    },
    utils::time,
};
use thiserror::Error as ThisError;

///
/// SaveError
///

#[derive(Debug, ThisError)]
pub enum SaveError {
    #[error(transparent)]
    ResolverError(#[from] ResolverError),

    #[error("key exists: {0}")]
    KeyExists(SortKey),

    #[error("key not found: {0}")]
    KeyNotFound(SortKey),
}

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
        let res = self.execute_internal(query).map_err(ServiceError::from)?;

        Ok(res)
    }

    // execute_internal
    fn execute_internal(&self, query: SaveQueryPrepared) -> Result<SaveResponse, StorageError> {
        let mode = query.mode;
        let entity = &*query.entity;

        //
        // build key / value
        //

        let ck = entity.composite_key();
        let sk = with_resolver(|r| r.resolve_sort_key(&entity.path_dyn(), &ck))
            .map_err(StorageError::from)?;

        // debug
        self.debug.println(&format!("query.{mode}: {sk}"));

        // validate
        let adapter = crate::visit::EntityAdapter(entity);
        crate::validate(&adapter)?;

        // serialize
        let data: Vec<u8> = entity.serialize()?;

        //
        // match mode
        // on Update and Replace compare old and new data
        //

        let now = time::now_secs();
        let store = self.data.with(|data| data.try_get_store(&entity.store()))?;
        let result = store.with_borrow(|store| store.get(&sk));

        let (created, modified) = match mode {
            SaveMode::Create => {
                if result.is_some() {
                    Err(StorageError::from(SaveError::KeyExists(sk.clone())))?;
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
                None => Err(StorageError::from(SaveError::KeyNotFound(sk.clone())))?,
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

        // prepare data value
        let path = entity.path_dyn();
        let value = DataValue {
            data,
            path,
            metadata: Metadata { created, modified },
        };

        // insert data row
        store.with_borrow_mut(|store| {
            store.data.insert(sk.clone(), value.clone());
        });

        Ok(SaveResponse {
            key: sk,
            created,
            modified,
        })
    }
}
