use crate::{
    node::{Def, MacroNode, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Sanitizer
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Sanitizer {
    pub def: Def,
}

impl MacroNode for Sanitizer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Sanitizer {}

impl VisitableNode for Sanitizer {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
    }
}
