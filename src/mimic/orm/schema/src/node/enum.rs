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

impl ValidateNode for Enum {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // check variants for unspecified
        let mut un_count = 0;
        let mut un_first = None;
        for (i, variant) in self.variants.iter().enumerate() {
            if variant.unspecified {
                un_count += 1;
                if un_first.is_none() {
                    un_first = Some(i);
                }
            }
        }

        // Check if there's more than one unspecified variant
        if un_count > 1 {
            errs.add("there should not be more than one unspecified variant");
        }

        // Check if the unspecified variant is not the first in the list
        if let Some(index) = un_first {
            if index != 0 {
                errs.add("the unspecified variant must be the first in the list");
            }
        }

        errs.result()
    }
}

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
/// EnumVariant
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discriminant: Option<i32>,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub default: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub unspecified: bool,
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
