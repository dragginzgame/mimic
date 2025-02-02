use crate::{
    orm::traits::Entity,
    query::{
        save::{save, SaveError, SaveMode},
        DebugContext,
    },
    store::StoreLocal,
};
use std::marker::PhantomData;

///
/// SaveBuilder
///

pub struct SaveBuilder<E>
where
    E: Entity,
{
    mode: SaveMode,
    debug: DebugContext,
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
    pub fn from_data(self, data: &[u8]) -> Result<SaveQuery<E>, SaveError> {
        let entity: E = crate::orm::deserialize(data)?;

        Ok(SaveQuery::new(self, vec![entity]))
    }

    // from_entity
    pub fn from_entity(self, entity: E) -> SaveQuery<E> {
        SaveQuery::new(self, vec![entity])
    }

    // from_entities
    #[must_use]
    pub const fn from_entities(self, entities: Vec<E>) -> SaveQuery<E> {
        SaveQuery::new(self, entities)
    }
}

///
/// SaveQuery
///

pub struct SaveQuery<E>
where
    E: Entity,
{
    mode: SaveMode,
    debug: DebugContext,
    entities: Vec<E>,
}

impl<E> SaveQuery<E>
where
    E: Entity,
{
    #[must_use]
    const fn new(builder: SaveBuilder<E>, entities: Vec<E>) -> Self {
        Self {
            mode: builder.mode,
            debug: builder.debug,
            entities,
        }
    }

    // execute
    pub fn execute(self, store: StoreLocal) -> Result<(), SaveError> {
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
    pub fn execute(self, store: StoreLocal) -> Result<(), SaveError> {
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

        for entity in entities {
            save(store, &mode, &debug, Box::new(entity))?;
        }

        Ok(())
    }
}
