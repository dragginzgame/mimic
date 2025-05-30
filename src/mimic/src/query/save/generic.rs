use crate::{
    Error,
    db::DbLocal,
    query::{
        DebugContext,
        save::{SaveMode, save},
    },
    serialize,
    traits::Entity,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

///
/// SaveBuilder
///

pub struct SaveBuilder<E>
where
    E: Entity,
{
    mode: SaveMode,
    phantom: PhantomData<E>,
}

impl<E> SaveBuilder<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub const fn new(mode: SaveMode) -> Self {
        Self {
            mode,
            phantom: PhantomData,
        }
    }

    // from_entity
    pub fn from_entity(self, entity: E) -> Result<SaveQuery, Error> {
        let bytes = serialize(&entity)?;

        Ok(SaveQuery::new(E::PATH, self.mode, bytes))
    }
}

///
/// SaveQueryBuilder
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize)]
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
    pub fn execute<E>(self, db: DbLocal) -> Result<SaveResponse, Error>
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
    pub fn execute<E>(self, db: DbLocal) -> Result<SaveResponse, Error>
    where
        E: Entity,
    {
        // Validate all entities first
        let entity: E = crate::deserialize(&self.query.bytes)?;
        let adapter = crate::visit::EntityAdapter(&entity);
        crate::validate(&adapter)?;

        // save entities
        save(db, self.query.mode, &self.query.debug, Box::new(entity))?;

        Ok(SaveResponse())
    }
}

///
/// SaveResponse
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct SaveResponse();
