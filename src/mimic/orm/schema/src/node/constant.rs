use crate::{
    node::{Arg, Def, MacroNode, ValidateNode, VisitableNode},
    types::PrimitiveType,
};
use serde::{Deserialize, Serialize};

//
// Constant
//

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Constant {
    pub def: Def,
    pub ty: PrimitiveType,
    pub value: Arg,
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
