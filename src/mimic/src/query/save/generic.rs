use crate::{
    Error,
    db::DbLocal,
    orm::{serialize, traits::Entity},
    query::{
        DebugContext, QueryError,
        save::{SaveMode, save},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// SaveBuilder
///

pub struct SaveBuilder {
    path: String,
    mode: SaveMode,
}

impl SaveBuilder {
    // new
    #[must_use]
    pub fn new(path: &str, mode: SaveMode) -> Self {
        Self {
            path: path.to_string(),
            mode,
        }
    }

    // from_entity
    pub fn from_entity<E: Entity>(self, entity: E) -> Result<SaveQuery, Error> {
        let bytes = serialize(&entity)?;

        Ok(SaveQuery::new(&self.path, self.mode, bytes))
    }

    // from_bytes
    pub fn from_bytes(self, bytes: &[u8]) -> Result<SaveQuery, Error> {
        Ok(SaveQuery::new(&self.path, self.mode, bytes.to_vec()))
    }
}

///
/// SaveQuery
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct SaveQuery {
    pub path: String,
    pub mode: SaveMode,
    pub bytes: Vec<u8>,
    pub debug: DebugContext,
}

impl SaveQuery {
    // new
    #[must_use]
    pub fn new(path: &str, mode: SaveMode, bytes: Vec<u8>) -> Self {
        Self {
            path: path.to_string(),
            mode,
            bytes,
            debug: DebugContext::default(),
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // execute
    pub fn execute<E>(self, db: DbLocal) -> Result<(), Error>
    where
        E: Entity,
    {
        let executor = SaveExecutor::new(self);

        executor.execute::<E>(db)
    }
}

///
/// SaveExecutor
///

pub struct SaveExecutor {
    query: SaveQuery,
}

impl SaveExecutor {
    // new
    #[must_use]
    pub const fn new(query: SaveQuery) -> Self {
        Self { query }
    }

    // execute
    pub fn execute<E>(self, db: DbLocal) -> Result<(), Error>
    where
        E: Entity,
    {
        // Validate all entities first
        let entity: E = crate::orm::deserialize(&self.query.bytes)?;
        let adapter = crate::orm::visit::EntityAdapter(&entity);
        crate::orm::validate(&adapter)?;

        // save entities
        save(db, &self.query.mode, &self.query.debug, Box::new(entity))
            .map_err(QueryError::SaveError)?;

        Ok(())
    }
}

///
/// SaveResponse
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct SaveResponse();
