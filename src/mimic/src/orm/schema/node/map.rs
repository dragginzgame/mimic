use crate::orm::schema::{
    node::{Def, Item, MacroNode, TypeValidator, ValidateNode, Value, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Map
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub def: Def,
    pub key: Item,
    pub value: Value,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validators: Vec<TypeValidator>,
}

impl MacroNode for Map {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Map {}

impl VisitableNode for Map {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        self.key.accept(v);
        self.value.accept(v);
        for node in &self.validators {
            node.accept(v);
        }
    }
}
