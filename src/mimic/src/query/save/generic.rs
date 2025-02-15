use crate::{
    Error,
    db::StoreLocal,
    orm::{deserialize, traits::Entity},
    query::{
        DebugContext, QueryError,
        save::{SaveError, SaveMode, save},
    },
};
use candid::CandidType;
use serde::Serialize;
use std::marker::PhantomData;

///
/// SaveBuilder
///

pub struct SaveBuilder<E>
where
    E: Entity,
{
    mode: SaveMode,
    _phantom: PhantomData<E>,
}

impl<E> SaveBuilder<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new(mode: SaveMode) -> Self {
        Self {
            mode,
            _phantom: PhantomData,
        }
    }

    // from_data
    pub fn from_data(self, data: &[u8]) -> Result<SaveQuery<E>, Error> {
        let entity: E = deserialize(data)
            .map_err(SaveError::OrmError)
            .map_err(QueryError::SaveError)?;

        Ok(SaveQuery::new(self.mode, vec![entity]))
    }

    // from_entity
    pub fn from_entity(self, entity: E) -> SaveQuery<E> {
        SaveQuery::new(self.mode, vec![entity])
    }

    // from_entities
    #[must_use]
    pub fn from_entities(self, entities: Vec<E>) -> SaveQuery<E> {
        SaveQuery::new(self.mode, entities)
    }
}

///
/// SaveQuery
///

#[derive(CandidType, Debug, Serialize)]
pub struct SaveQuery<E>
where
    E: Entity,
{
    mode: SaveMode,
    entities: Vec<E>,
    debug: DebugContext,
}

impl<E> SaveQuery<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new(mode: SaveMode, entities: Vec<E>) -> Self {
        Self {
            mode,
            entities,
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
    pub fn execute(self, store: StoreLocal) -> Result<(), Error> {
        let executor = SaveExecutor::new(self);

        executor.execute(store)
    }
}

///
/// SaveExecutor
///

pub struct SaveExecutor<E>
where
    E: Entity,
{
    query: SaveQuery<E>,
}

impl<E> SaveExecutor<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub const fn new(query: SaveQuery<E>) -> Self {
        Self { query }
    }

    // execute
    pub fn execute(self, store: StoreLocal) -> Result<(), Error> {
        // Validate all entities first
        for entity in &self.query.entities {
            crate::orm::validate(entity)?;
        }

        // Extract the mode, debug, etc. from self.query if needed
        let mode = self.query.mode;
        let debug = self.query.debug;
        let entities = self.query.entities;

        for entity in entities {
            save(store, &mode, &debug, Box::new(entity)).map_err(QueryError::SaveError)?;
        }

        Ok(())
    }
}
