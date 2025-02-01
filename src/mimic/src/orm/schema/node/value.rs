use crate::orm::schema::{
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
    fn drive<V: Visitor>(&self, v: &mut V) {
        self.item.accept(v);
        if let Some(node) = &self.default {
            node.accept(v);
        }
    }
}
