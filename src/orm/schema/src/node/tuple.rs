use crate::{
    node::{Def, MacroNode, ValidateNode, Value, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Tuple
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tuple {
    pub def: Def,
    pub values: Vec<Value>,
}

impl MacroNode for Tuple {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Tuple {}

impl VisitableNode for Tuple {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        for node in &self.values {
            node.accept(v);
        }
    }
}
