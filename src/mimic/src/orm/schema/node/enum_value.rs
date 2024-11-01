use crate::{
    orm::schema::node::{Def, MacroNode, ValidateNode, VisitableNode, Visitor},
    types::ErrorVec,
    utils::case::{Case, Casing},
};
use serde::{Deserialize, Serialize};
use std::ops::Not;

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
    pub value: i64,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub default: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub unspecified: bool,
}

impl ValidateNode for EnumValueVariant {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

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
