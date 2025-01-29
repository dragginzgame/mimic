use crate::{
    db::store::StoreLocal,
    orm::traits::{Entity, EntityDyn},
    query::{
        save::{save, SaveError, SaveMode},
        DebugContext,
    },
};
use std::mem;

///
/// SaveBuilder
///

pub struct SaveBuilder {
    mode: SaveMode,
    debug: DebugContext,
}

impl SaveBuilder {
    // new
    #[must_use]
    pub fn new(mode: SaveMode) -> Self {
        Self {
            mode,
            debug: DebugContext::default(),
        }
    }

    // debug
    #[must_use]
    pub fn debug(mut self) -> Self {
        self.debug.enable();
        self
    }

    // from_data
    pub fn from_data<E: Entity + 'static>(self, data: &[u8]) -> Result<SaveQuery, SaveError> {
        let entity: E = crate::orm::deserialize(data)?;

        Ok(SaveQuery::new(self, vec![Box::new(entity)]))
    }

    // from_entity
    pub fn from_entity<E: EntityDyn + 'static>(self, entity: E) -> SaveQuery {
        SaveQuery::new(self, vec![Box::new(entity)])
    }

    // from_entities
    #[must_use]
    pub fn from_entities<E: EntityDyn + 'static>(self, entities: Vec<E>) -> SaveQuery {
        let boxed_entities = entities
            .into_iter()
            .map(|entity| Box::new(entity) as Box<dyn EntityDyn>)
            .collect();

        SaveQuery::new(self, boxed_entities)
    }

    // from_entity_dynamic
    #[must_use]
    pub fn from_entity_dynamic(self, entity: Box<dyn EntityDyn>) -> SaveQuery {
        SaveQuery::new(self, vec![entity])
    }

    // from_entities_dynamic
    #[must_use]
    pub fn from_entities_dynamic(self, entities: Vec<Box<dyn EntityDyn>>) -> SaveQuery {
        SaveQuery::new(self, entities)
    }
}

///
/// SaveQuery
///

pub struct SaveQuery {
    mode: SaveMode,
    debug: DebugContext,
    entities: Vec<Box<dyn EntityDyn>>,
}

impl SaveQuery {
    #[must_use]
    pub fn new(builder: SaveBuilder, entities: Vec<Box<dyn EntityDyn>>) -> Self {
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
    pub fn execute(mut self, store: StoreLocal) -> Result<(), SaveError> {
        // Validate all entities first
        for entity in &self.query.entities {
            let adapter = crate::orm::visit::EntityAdapter(&**entity);
            crate::orm::validate(&adapter).map_err(|e| SaveError::Validation {
                path: entity.path_dyn(),
                source: e,
            })?;
        }

        // Temporarily take the entities out of self to avoid borrowing issues
        let mode = self.query.mode;
        let debug = self.query.debug;
        let entities = mem::take(&mut self.query.entities);

        // save entities
        for entity in entities {
            save(store, &mode, &debug, entity)?;
        }

        Ok(())
    }
}
