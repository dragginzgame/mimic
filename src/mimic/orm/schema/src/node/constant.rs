use crate::node::{Def, MacroNode, ValidateNode, VisitableNode};
use serde::{Deserialize, Serialize};

//
// Constant
//

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Constant {
    pub def: Def,
}

impl MacroNode for Constant {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Constant {}

impl VisitableNode for Constant {
    fn route_key(&self) -> String {
        self.def.path()
    }
}
