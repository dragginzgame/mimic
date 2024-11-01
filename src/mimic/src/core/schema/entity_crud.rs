use super::get_schema;
use crate::orm::schema::node::{Crud, Entity, Store};
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{collections::HashMap, sync::LazyLock};

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Schema {
        source: crate::orm::schema::node::Error,
    },
}

///
/// EntityCrudMap
/// internal static HashMap
///

static ENTITY_CRUD_MAP: LazyLock<EntityCrudMap> = LazyLock::new(EntityCrudMap::init);

#[derive(Clone, Debug, Default, Deref)]
pub struct EntityCrudMap(HashMap<String, Crud>);

impl EntityCrudMap {
    #[must_use]
    fn init() -> Self {
        let mut map = HashMap::new();

        for (entity_path, entity) in get_schema().unwrap().get_nodes::<Entity>() {
            map.insert(entity_path.to_string(), Self::get_crud(entity).unwrap());
        }

        Self(map)
    }

    // get_crud
    fn get_crud(entity: &Entity) -> Result<Crud, Error> {
        let schema = get_schema().unwrap();
        let store = schema
            .try_get_node::<Store>(&entity.store)
            .map_err(Error::from)?;

        // entity overrides store
        entity
            .crud
            .as_ref()
            .map_or_else(|| Ok(store.crud.clone()), |ec| Ok(ec.clone()))
    }
}

///
/// EntityCrudManager
///

pub struct EntityCrudManager {}

impl EntityCrudManager {
    pub fn get(entity: &str) -> Option<&Crud> {
        ENTITY_CRUD_MAP.get(entity)
    }
}
