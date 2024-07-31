use crate::{
    node::{Arg, Item, ValidateNode, VisitableNode},
    types::Cardinality,
    visit::Visitor,
};
use serde::{Deserialize, Serialize};

///
/// Value
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    pub cardinality: Cardinality,
    pub item: Item,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Arg>,
}

impl ValidateNode for Value {}

impl VisitableNode for Value {
    fn route_key(&self) -> String {
        match self.cardinality {
            Cardinality::One => "one",
            Cardinality::Opt => "opt",
            Cardinality::Many => "vec",
        }
        .to_string()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.item.accept(v);
        if let Some(node) = &self.default {
            node.accept(v);
        }
    }
}
