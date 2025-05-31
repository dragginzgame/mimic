use crate::{
    Error, ThisError,
    db::{
        DataStoreRegistryLocal,
        types::{DataValue, Metadata, SortKey},
    },
    deserialize,
    query::{DebugContext, QueryError, resolver::Resolver},
    traits::{Entity, EntityDyn},
};
use candid::CandidType;
use derive_more::Display;
use serde::{Deserialize, Serialize};

///
/// SaveError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum SaveError {
    #[error("key exists: {0}")]
    KeyExists(SortKey),

    #[error("key not found: {0}")]
    KeyNotFound(SortKey),
}

///
/// SaveMode
///
/// Create  : will only insert a row if it's empty
/// Replace : will change the row regardless of what was there
/// Update  : will only change an existing row
///

#[derive(CandidType, Clone, Copy, Debug, Default, Display, Serialize, Deserialize)]
pub enum SaveMode {
    #[default]
    Create,
    Replace,
    Update,
}

///
/// SaveQuery
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
pub struct SaveQuery {
    pub mode: SaveMode,
    pub bytes: Vec<u8>,
}

impl SaveQuery {
    #[must_use]
    pub fn new(mode: SaveMode, bytes: &[u8]) -> Self {
        Self {
            mode,
            bytes: bytes.to_vec(),
        }
    }
}

///
/// SaveQueryInit
///

#[derive(Default)]
pub struct SaveQueryInit {}

impl SaveQueryInit {
    // new
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    // query
    pub fn query<E: Entity + 'static>(self, query: SaveQuery) -> Result<SaveQueryBuilder, Error> {
        self.bytes::<E>(query.mode, &query.bytes)
    }

    // bytes
    pub fn bytes<E: Entity + 'static>(
        self,
        mode: SaveMode,
        bytes: &[u8],
    ) -> Result<SaveQueryBuilder, Error> {
        let entity = deserialize::<E>(bytes)?;

        Ok(SaveQueryBuilder::new(mode, Box::new(entity)))
    }

    // entity
    pub fn entity<E: Entity + 'static>(self, mode: SaveMode, entity: E) -> SaveQueryBuilder {
        SaveQueryBuilder::new(mode, Box::new(entity))
    }

    // entity_dyn
    #[must_use]
    pub fn entity_dyn(self, mode: SaveMode, entity: Box<dyn EntityDyn>) -> SaveQueryBuilder {
        SaveQueryBuilder::new(mode, entity)
    }
}

///
/// SaveQueryModeInit
///

#[derive(Default)]
pub struct SaveQueryModeInit {
    mode: SaveMode,
}

impl SaveQueryModeInit {
    // new
    #[must_use]
    pub const fn new(mode: SaveMode) -> Self {
        Self { mode }
    }

    // bytes
    pub fn bytes<E: Entity + 'static>(self, bytes: &[u8]) -> Result<SaveQueryBuilder, Error> {
        let entity = deserialize::<E>(bytes)?;

        Ok(SaveQueryBuilder::new(self.mode, Box::new(entity)))
    }

    // entity
    pub fn entity<E: Entity + 'static>(self, entity: E) -> SaveQueryBuilder {
        SaveQueryBuilder::new(self.mode, Box::new(entity))
    }

    // entity_dyn
    #[must_use]
    pub fn entity_dyn(self, entity: Box<dyn EntityDyn>) -> SaveQueryBuilder {
        SaveQueryBuilder::new(self.mode, entity)
    }
}

///
/// SaveQueryBuilder
///

pub struct SaveQueryBuilder {
    mode: SaveMode,
    entity: Box<dyn EntityDyn>,
    debug: DebugContext,
}

impl SaveQueryBuilder {
    // new
    #[must_use]
    pub fn new(mode: SaveMode, entity: Box<dyn EntityDyn>) -> Self {
        Self {
            mode,
            entity,
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
    pub fn execute(self, db: DataStoreRegistryLocal) -> Result<SaveResponse, Error> {
        let executor = SaveQueryExecutor::new(self.mode, self.entity, self.debug);

        executor.execute(db)
    }
}

///
/// SaveQueryExecutor
///

pub struct SaveQueryExecutor {
    mode: SaveMode,
    entity: Box<dyn EntityDyn>,
    debug: DebugContext,
}

impl SaveQueryExecutor {
    // new
    #[must_use]
    pub fn new(mode: SaveMode, entity: Box<dyn EntityDyn>, debug: DebugContext) -> Self {
        Self {
            mode,
            entity,
            debug,
        }
    }

    // execute
    pub fn execute(self, db: DataStoreRegistryLocal) -> Result<SaveResponse, Error> {
        // Validate all entities first
        let adapter = crate::visit::EntityAdapter(&*self.entity);
        crate::validate(&adapter)?;

        // save entities
        let resp = save(db, self.mode, &self.debug, self.entity)?;

        Ok(resp)
    }
}

///
/// SaveResponse
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct SaveResponse();

// save
fn save<'a>(
    db: DataStoreRegistryLocal,
    mode: SaveMode,
    debug: &DebugContext,
    entity: Box<dyn EntityDyn + 'a>,
) -> Result<SaveResponse, QueryError> {
    //
    // build key / value
    //

    let ck = entity.composite_key();
    let resolver = Resolver::new(&entity.path_dyn());
    let key = resolver.data_key(&ck).map_err(QueryError::from)?;

    // debug
    debug.println(&format!("query.{mode}: {key}",));

    // serialize
    let data: Vec<u8> = entity.serialize()?;

    //
    // match mode
    // on Update and Replace compare old and new data
    //

    let now = crate::utils::time::now_secs();
    let store = db
        .with(|db| db.try_get_store(&entity.store()))
        .map_err(QueryError::from)?;
    let result = store.with_borrow(|store| store.get(&key));

    let (created, modified) = match mode {
        SaveMode::Create => {
            if result.is_some() {
                #[allow(clippy::redundant_clone)]
                Err(SaveError::KeyExists(key.clone()))?;
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
            None => Err(SaveError::KeyNotFound(key.clone()))?,
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
        store.data.insert(key.clone(), value.clone());
    });

    Ok(SaveResponse())
}
