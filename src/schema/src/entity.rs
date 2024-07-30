use crate::SCHEMA;
use candid::CandidType;
use derive_more::Deref;
use schema::node::{Crud, Entity, Store};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{collections::HashMap, sync::LazyLock};

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Schema { source: schema::node::Error },
}

pub static ENTITY_CRUD_MAP: LazyLock<EntityCrudMap> = LazyLock::new(EntityCrudMap::init);

///
/// EntityCrudMap
///

#[derive(Clone, Debug, Default, Deref)]
pub struct EntityCrudMap(HashMap<String, Crud>);

impl EntityCrudMap {
    pub fn init() -> Self {
        let mut map = HashMap::new();

        for (entity_path, entity) in SCHEMA.get_nodes::<Entity>() {
            map.insert(entity_path.to_string(), Self::get_crud(entity).unwrap());
        }

        Self(map)
    }

    // get_crud
    fn get_crud(entity: &Entity) -> Result<Crud, Error> {
        let store = SCHEMA.try_get_node::<Store>(&entity.store)?;

        // entity overrides store
        if let Some(entity_crud) = &entity.crud {
            Ok(entity_crud.clone())
        } else {
            Ok(store.crud.clone())
        }
    }
}
