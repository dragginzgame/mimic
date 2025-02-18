use crate::{
    Error,
    db::DbLocal,
    orm::traits::Entity,
    query::{
        DebugContext, QueryError,
        save::{SaveMode, save},
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// SaveBuilderDyn
///

pub struct SaveBuilderDyn {
    path: String,
    mode: SaveMode,
}

impl SaveBuilderDyn {
    // new
    #[must_use]
    pub fn new(path: &str, mode: SaveMode) -> Self {
        Self {
            path: path.to_string(),
            mode,
        }
    }

    // from_bytes
    pub fn from_bytes(self, bytes: &[u8]) -> Result<SaveQueryDyn, Error> {
        Ok(SaveQueryDyn::new(&self.path, self.mode, bytes.to_vec()))
    }
}

///
/// SaveQueryDyn
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct SaveQueryDyn {
    pub path: String,
    pub mode: SaveMode,
    pub bytes: Vec<u8>,
    pub debug: DebugContext,
}

impl SaveQueryDyn {
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
        let executor = SaveExecutorDyn::new(self);

        executor.execute::<E>(db)
    }
}

///
/// SaveExecutorDyn
///

pub struct SaveExecutorDyn {
    query: SaveQueryDyn,
}

impl SaveExecutorDyn {
    // new
    #[must_use]
    pub const fn new(query: SaveQueryDyn) -> Self {
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
/// SaveResponseDyn
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct SaveResponseDyn();
