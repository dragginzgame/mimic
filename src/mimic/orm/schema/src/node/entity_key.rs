use crate::{
    build::schema_read,
    node::{Def, Entity, MacroNode, ValidateNode, VisitableNode},
};
use serde::{Deserialize, Serialize};
use types::ErrorVec;

///
/// EntityKey
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityKey {
    pub def: Def,
    pub entity: String,

    // keys are needed because the schema has to check for duplicate values
    // of entity-key, we just don't need them in the schema.json
    #[serde(default, skip_serializing)]
    pub keys: Vec<String>,
}

impl MacroNode for EntityKey {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for EntityKey {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // fixtures
        errs.add_result(schema_read().check_node::<Entity>(&self.entity));

        errs.result()
    }
}

impl VisitableNode for EntityKey {
    fn route_key(&self) -> String {
        self.def.path()
    }
}
