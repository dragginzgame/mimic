use crate::{
    node::{DefNode, DefStruct, Item, TypeValidator, ValidateNode, VisitableNode},
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Set
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Set {
    pub def: DefStruct,
    pub item: Item,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validators: Vec<TypeValidator>,
}

impl ValidateNode for Set {}

impl VisitableNode for Set {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        self.item.accept(v);
        for node in &self.validators {
            node.accept(v);
        }
    }
}
