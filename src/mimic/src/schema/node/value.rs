use crate::schema::{
    node::{Item, ValidateNode, VisitableNode},
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
}

impl ValidateNode for Value {}

impl VisitableNode for Value {
    fn drive<V: Visitor>(&self, v: &mut V) {
        self.item.accept(v);
    }
}
