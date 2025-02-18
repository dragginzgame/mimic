use crate::{
    Error,
    db::DbLocal,
    orm::{
        deserialize,
        traits::{Entity, EntityDyn},
    },
    query::{
        DebugContext, QueryError,
        save::{SaveError, SaveMode, SaveResponse, save},
    },
};
use std::mem;

///
/// SaveBuilder
///

pub struct SaveBuilder {
    mode: SaveMode,
}

impl SaveBuilder {
    // new
    #[must_use]
    pub const fn new(mode: SaveMode) -> Self {
        Self { mode }
    }

    // from_bytes
    pub fn from_bytes<E: Entity + 'static>(self, data: &[u8]) -> Result<SaveQuery, Error> {
        let entity: E = deserialize(data)
            .map_err(SaveError::OrmError)
            .map_err(QueryError::SaveError)?;

        Ok(SaveQuery::new(self.mode, vec![Box::new(entity)]))
    }

    // from_entity
    pub fn from_entity<E: Entity + 'static>(self, entity: E) -> SaveQuery {
        SaveQuery::new(self.mode, vec![Box::new(entity)])
    }

    // from_entities
    #[must_use]
    pub fn from_entities<E: Entity + 'static>(self, entities: Vec<E>) -> SaveQuery {
        let boxed_entities = entities
            .into_iter()
            .map(|entity| Box::new(entity) as Box<dyn EntityDyn>)
            .collect();

        SaveQuery::new(self.mode, boxed_entities)
    }

    // from_entity_dyn
    #[must_use]
    pub fn from_entity_dyn(self, entity: Box<dyn EntityDyn>) -> SaveQuery {
        SaveQuery::new(self.mode, vec![entity])
    }

    // from_entities_dyn
    #[must_use]
    pub fn from_entities_dyn(self, entities: Vec<Box<dyn EntityDyn>>) -> SaveQuery {
        SaveQuery::new(self.mode, entities)
    }
}

///
/// SaveQuery
///

#[derive(Debug)]
pub struct SaveQuery {
    mode: SaveMode,
    entities: Vec<Box<dyn EntityDyn>>,
    debug: DebugContext,
}

impl SaveQuery {
    // new
    #[must_use]
    pub fn new(mode: SaveMode, entities: Vec<Box<dyn EntityDyn>>) -> Self {
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
    pub fn execute(self, db: DbLocal) -> Result<SaveResponse, Error> {
        let executor = SaveExecutor::new(self);

        executor.execute(db)
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
    pub fn execute(mut self, db: DbLocal) -> Result<SaveResponse, Error> {
        // Validate all entities first
        for entity in &self.query.entities {
            let adapter = crate::orm::visit::EntityAdapter(&**entity);
            crate::orm::validate(&adapter)?;
        }

        // Temporarily take the entities out of self to avoid borrowing issues
        let mode = self.query.mode;
        let debug = self.query.debug;
        let entities = mem::take(&mut self.query.entities);

        // save entities
        for entity in entities {
            save(db, &mode, &debug, entity).map_err(QueryError::SaveError)?;
        }

        Ok(SaveResponse())
    }
}
