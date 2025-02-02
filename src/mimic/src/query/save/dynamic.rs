use crate::{
    orm::traits::{Entity, EntityDyn},
    query::{
        save::{save, SaveError, SaveMode},
        DebugContext,
    },
    store::StoreLocal,
};
use std::mem;

///
/// SaveBuilderDyn
///

pub struct SaveBuilderDyn {
    mode: SaveMode,
    debug: DebugContext,
}

impl SaveBuilderDyn {
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
    pub fn from_data<E: Entity + 'static>(self, data: &[u8]) -> Result<SaveQueryDyn, SaveError> {
        let entity: E = crate::orm::deserialize(data)?;

        Ok(SaveQueryDyn::new(self, vec![Box::new(entity)]))
    }

    // from_entity
    pub fn from_entity<E: EntityDyn + 'static>(self, entity: E) -> SaveQueryDyn {
        SaveQueryDyn::new(self, vec![Box::new(entity)])
    }

    // from_entities
    #[must_use]
    pub fn from_entities<E: EntityDyn + 'static>(self, entities: Vec<E>) -> SaveQueryDyn {
        let boxed_entities = entities
            .into_iter()
            .map(|entity| Box::new(entity) as Box<dyn EntityDyn>)
            .collect();

        SaveQueryDyn::new(self, boxed_entities)
    }

    // from_entity_dynamic
    #[must_use]
    pub fn from_entity_dynamic(self, entity: Box<dyn EntityDyn>) -> SaveQueryDyn {
        SaveQueryDyn::new(self, vec![entity])
    }

    // from_entities_dynamic
    #[must_use]
    pub fn from_entities_dynamic(self, entities: Vec<Box<dyn EntityDyn>>) -> SaveQueryDyn {
        SaveQueryDyn::new(self, entities)
    }
}

///
/// SaveQueryDyn
///

pub struct SaveQueryDyn {
    mode: SaveMode,
    debug: DebugContext,
    entities: Vec<Box<dyn EntityDyn>>,
}

impl SaveQueryDyn {
    #[must_use]
    pub fn new(builder: SaveBuilderDyn, entities: Vec<Box<dyn EntityDyn>>) -> Self {
        Self {
            mode: builder.mode,
            debug: builder.debug,
            entities,
        }
    }

    // execute
    pub fn execute(self, store: StoreLocal) -> Result<(), SaveError> {
        let executor = SaveExecutorDyn::new(self);

        executor.execute(store)
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
