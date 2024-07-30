use crate::{
    node::{Def, MacroNode, ValidateNode, Value, VisitableNode},
    visit::Visitor,
};
use lib_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::ops::Not;
use types::ErrorVec;

///
/// Enum
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enum {
    pub def: Def,
    pub variants: Vec<EnumVariant>,
}

impl MacroNode for Enum {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for Enum {}

impl VisitableNode for Enum {
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
/// EnumDiscriminant
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnumDiscriminant {
    I32(i32),
    Hash,
}

///
/// EnumVariant
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discriminant: Option<EnumDiscriminant>,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub default: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub invalid: bool,
}

impl ValidateNode for EnumVariant {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // name
        if !self.name.is_case(Case::UpperCamel) {
            errs.add(format!(
                "variant name '{}' must be in UpperCamelCase",
                self.name
            ));
        }

        // value + discriminant
        if self.value.is_some() && self.discriminant.is_some() {
            errs.add("cannot set both a value and a discriminant");
        }

        errs.result()
    }
}

impl VisitableNode for EnumVariant {
    fn drive<V: Visitor>(&self, v: &mut V) {
        if let Some(node) = &self.value {
            node.accept(v);
        }
    }
}
