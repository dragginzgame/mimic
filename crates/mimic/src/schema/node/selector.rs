use crate::{
    schema::node::{Arg, Def, MacroNode, ValidateNode, VisitableNode, Visitor},
    types::ErrorTree,
    utils::case::{Case, Casing},
};
use serde::{Deserialize, Serialize};
use std::ops::Not;

///
/// Selector
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Selector {
    pub def: Def,
    pub target: String,

    #[serde(default)]
    pub variants: Vec<SelectorVariant>,
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
        for node in &self.variants {
            node.accept(v);
        }
    }
}

///
/// SelectorVariant
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectorVariant {
    pub name: String,
    pub value: Arg,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub default: bool,
}

impl ValidateNode for SelectorVariant {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::default();

        // name
        if !self.name.is_case(Case::UpperCamel) {
            errs.add(format!(
                "variant name '{}' must be in UpperCamelCase",
                self.name
            ));
        }

        errs.result()
    }
}

impl VisitableNode for SelectorVariant {}
