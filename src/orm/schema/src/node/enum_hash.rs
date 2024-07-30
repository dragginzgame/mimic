use crate::node::{Def, MacroNode, ValidateNode, VisitableNode};
use serde::{Deserialize, Serialize};

///
/// EnumHash
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumHash {
    pub def: Def,

    #[serde(default, skip_serializing)]
    pub keys: Vec<String>,
}

impl MacroNode for EnumHash {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for EnumHash {}

impl VisitableNode for EnumHash {
    fn route_key(&self) -> String {
        self.def.path()
    }
}
