use crate::schema::{
    node::{Def, MacroNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::Serialize;

///
/// Validator
///

#[derive(Clone, Debug, Serialize)]
pub struct Validator {
    pub def: Def,
}

impl MacroNode for Validator {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Validator {}

impl VisitableNode for Validator {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
    }
}
