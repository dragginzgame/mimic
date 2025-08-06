use crate::node::{Arg, Def, MacroNode, ValidateNode, VisitableNode, Visitor};
use serde::Serialize;
use std::ops::Not;

///
/// Selector
///

#[derive(Clone, Debug, Serialize)]
pub struct Selector {
    pub def: Def,
    pub target: &'static str,
    pub variants: &'static [SelectorVariant],
}

impl MacroNode for Selector {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Selector {}

impl VisitableNode for Selector {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        for node in self.variants {
            node.accept(v);
        }
    }
}

///
/// SelectorVariant
///

#[derive(Clone, Debug, Serialize)]
pub struct SelectorVariant {
    pub name: &'static str,
    pub value: Arg,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub default: bool,
}

impl ValidateNode for SelectorVariant {}

impl VisitableNode for SelectorVariant {}
