use crate::schema::{
    node::{Arg, Def, MacroNode, ValidateNode, VisitableNode},
    types::ConstantType,
    visit::Visitor,
};
use serde::Serialize;

///
/// Constant
///

#[derive(Clone, Debug, Serialize)]
pub struct Constant {
    pub def: Def,
    pub ty: ConstantType,
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

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
    }
}
