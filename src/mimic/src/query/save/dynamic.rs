use crate::{
    Error,
    db::DbLocal,
    deserialize,
    query::{
        DebugContext,
        save::{SaveMode, save},
    },
    traits::{Entity, EntityDyn},
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::mem;

///
/// SaveBuilderDyn
///

pub struct SaveBuilderDyn {
    mode: SaveMode,
}

impl SaveBuilderDyn {
    // new
    #[must_use]
    pub const fn new(mode: SaveMode) -> Self {
        Self { mode }
    }

    // from_bytes
    pub fn from_bytes<E: Entity + 'static>(self, data: &[u8]) -> Result<SaveQueryDyn, Error> {
        let entity: E = deserialize(data)?;

        Ok(SaveQueryDyn::new(self.mode, vec![Box::new(entity)]))
    }

    // from_entity
    pub fn from_entity<E: Entity + 'static>(self, entity: E) -> SaveQueryDyn {
        SaveQueryDyn::new(self.mode, vec![Box::new(entity)])
    }

    // from_entities
    #[must_use]
    pub fn from_entities<E: Entity + 'static>(self, entities: Vec<E>) -> SaveQueryDyn {
        let boxed_entities = entities
            .into_iter()
            .map(|entity| Box::new(entity) as Box<dyn EntityDyn>)
            .collect();

        SaveQueryDyn::new(self.mode, boxed_entities)
    }

    // from_entity_dyn
    #[must_use]
    pub fn from_entity_dyn(self, entity: Box<dyn EntityDyn>) -> SaveQueryDyn {
        SaveQueryDyn::new(self.mode, vec![entity])
    }

    // from_entities_dyn
    #[must_use]
    pub fn from_entities_dyn(self, entities: Vec<Box<dyn EntityDyn>>) -> SaveQueryDyn {
        SaveQueryDyn::new(self.mode, entities)
    }
}

///
/// SaveQueryDyn
///

#[derive(Debug, Default)]
pub struct SaveQueryDyn {
    mode: SaveMode,
    entities: Vec<Box<dyn EntityDyn>>,
    debug: DebugContext,
}

impl SaveQueryDyn {
    // new
    #[must_use]
    pub fn new(mode: SaveMode, entities: Vec<Box<dyn EntityDyn>>) -> Self {
        Self {
            mode,
            entities,
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
    pub fn execute(self, db: DbLocal) -> Result<SaveResponseDyn, Error> {
        let executor = SaveExecutorDyn::new(self);

        executor.execute(db)
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
    pub fn execute(mut self, db: DbLocal) -> Result<SaveResponseDyn, Error> {
        // Validate all entities first
        for entity in &self.query.entities {
            let adapter = crate::visit::EntityAdapter(&**entity);
            crate::validate(&adapter)?;
        }

        // Temporarily take the entities out of self to avoid borrowing issues
        let mode = self.query.mode;
        let debug = self.query.debug;
        let entities = mem::take(&mut self.query.entities);

        // save entities
        for entity in entities {
            save(db, mode, &debug, entity)?;
        }

        Ok(SaveResponseDyn())
    }
}

///
/// SaveResponseDyn
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct SaveResponseDyn();
