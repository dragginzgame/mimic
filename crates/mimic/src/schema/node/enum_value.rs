use crate::{
    error::ErrorTree,
    schema::node::{
        ArgNumber, Def, MacroNode, Type, TypeNode, ValidateNode, VisitableNode, Visitor,
    },
    utils::case::{Case, Casing},
};
use serde::Serialize;
use std::ops::Not;

///
/// EnumValue
///

#[derive(Clone, Debug, Serialize)]
pub struct EnumValue {
    pub def: Def,

    #[serde(default)]
    pub variants: &'static [EnumValueVariant],

    pub ty: Type,
}

impl MacroNode for EnumValue {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TypeNode for EnumValue {
    fn ty(&self) -> &Type {
        &self.ty
    }
}

impl ValidateNode for EnumValue {}

impl VisitableNode for EnumValue {
    fn route_key(&self) -> String {
        self.def.path()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.def.accept(v);
        for node in self.variants {
            node.accept(v);
        }
        self.ty.accept(v);
    }
}

///
/// EnumValueVariant
///

#[derive(Clone, Debug, Serialize)]
pub struct EnumValueVariant {
    pub name: &'static str,
    pub value: ArgNumber,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub default: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub unspecified: bool,
}

impl ValidateNode for EnumValueVariant {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();

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
