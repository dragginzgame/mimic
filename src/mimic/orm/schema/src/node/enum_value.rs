use crate::node::{Def, MacroNode, ValidateNode, VisitableNode, Visitor};
use lib_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::ops::Not;
use types::ErrorVec;

///
/// EnumValue
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumValue {
    pub def: Def,

    #[serde(default)]
    pub variants: Vec<EnumValueVariant>,
}

impl MacroNode for EnumValue {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ValidateNode for EnumValue {}

impl VisitableNode for EnumValue {
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
/// EnumValueVariant
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumValueVariant {
    pub name: String,
    pub value: Option<i64>,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub default: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub unspecified: bool,
}

impl ValidateNode for EnumValueVariant {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // unspecified
        if self.value.is_some() == self.unspecified {
            errs.add("unspecified must be the only variant without a value");
        }

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

impl VisitableNode for EnumValueVariant {}
