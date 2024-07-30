use crate::{
    node::{Def, MacroNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Validator
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
