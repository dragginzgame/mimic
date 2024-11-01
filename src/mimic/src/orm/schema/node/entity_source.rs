use crate::orm::{
    schema::{
        build::schema_read,
        node::{Def, Entity, MacroNode, ValidateNode, VisitableNode},
    },
    types::ErrorVec,
};
use serde::{Deserialize, Serialize};

///
/// EntitySource
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntitySource {
    pub def: Def,
    pub entity: String,
    pub sources: Vec<EntitySourceEntry>,
}

impl MacroNode for EntitySource {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for EntitySource {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // fixtures
        errs.add_result(schema_read().check_node::<Entity>(&self.entity));

        errs.result()
    }
}

impl VisitableNode for EntitySource {
    fn route_key(&self) -> String {
        self.def.path()
    }
}

///
/// EntitySourceEntry
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntitySourceEntry {
    pub name: String,
    pub path: String,
}
