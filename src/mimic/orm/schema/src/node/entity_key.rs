use crate::node::{Def, MacroNode, ValidateNode, VisitableNode};
use serde::{Deserialize, Serialize};

///
/// EntityKey
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityKey {
    pub def: Def,

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

impl ValidateNode for EntityKey {}

impl VisitableNode for EntityKey {
    fn route_key(&self) -> String {
        self.def.path()
    }
}
