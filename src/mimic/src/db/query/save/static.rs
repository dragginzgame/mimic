use crate::{
    db::{
        query::{
            save::{save, SaveError, SaveMode},
            DebugContext,
        },
        Db,
    },
    orm::traits::Entity,
};
use std::marker::PhantomData;

///
/// ESaveBuilder
///

pub struct ESaveBuilder<E>
where
    E: Entity,
{
    mode: SaveMode,
    debug: DebugContext,
    _phantom: PhantomData<E>,
}

impl<E> ESaveBuilder<E>
where
    E: Entity,
{
    // new
    #[must_use]
    pub fn new(mode: SaveMode) -> Self {
        Self {
            mode,
            debug: DebugContext::default(),
            _phantom: PhantomData,
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // from_data
    pub fn from_data(self, data: &[u8]) -> Result<ESaveQuery<E>, SaveError> {
        let entity: E = crate::orm::deserialize(data)?;

        Ok(ESaveQuery::new(self, vec![entity]))
    }

    // from_entity
    pub fn from_entity(self, entity: E) -> ESaveQuery<E> {
        ESaveQuery::new(self, vec![entity])
    }

    // from_entities
    #[must_use]
    pub fn from_entities(self, entities: Vec<E>) -> ESaveQuery<E> {
        ESaveQuery::new(self, entities)
    }
}

///
/// ESaveQuery
///

pub struct ESaveQuery<E>
where
    E: Entity + 'static,
{
    mode: SaveMode,
    debug: DebugContext,
    entities: Vec<E>,
}

impl<E> ESaveQuery<E>
where
    E: Entity + 'static,
{
    #[must_use]
    fn new(builder: ESaveBuilder<E>, entities: Vec<E>) -> Self {
        Self {
            mode: builder.mode,
            debug: builder.debug,
            entities,
        }
    }

    // execute
    pub fn execute(self, db: &Db) -> Result<(), SaveError> {
        let executor = ESaveExecutor::new(self);

        executor.execute(db)
    }
}

///
/// ESaveExecutor
///

pub struct ESaveExecutor<E>
where
    E: Entity + 'static,
{
    query: ESaveQuery<E>,
}

impl<E> ESaveExecutor<E>
where
    E: Entity + 'static,
{
    // new
    #[must_use]
    pub fn new(query: ESaveQuery<E>) -> Self {
        Self { query }
    }

    // execute
    pub fn execute(self, db: &Db) -> Result<(), SaveError> {
        // Validate all entities first
        for entity in &self.query.entities {
            crate::orm::validate(entity).map_err(|e| SaveError::Validation {
                path: E::path(),
                source: e,
            })?;
        }

        // Extract the mode, debug, etc. from self.query if needed
        let mode = self.query.mode;
        let debug = self.query.debug;
        let entities = self.query.entities;

        for entity in entities.into_iter() {
            save(db, &mode, &debug, Box::new(entity))?;
        }

        Ok(())
    }
}
