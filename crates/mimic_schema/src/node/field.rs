use crate::{
    build::validate::validate_ident,
    node::{Arg, ValidateNode, Value, VisitableNode},
    types::Cardinality,
    visit::Visitor,
};
use mimic_common::{
    error::ErrorTree,
    utils::case::{Case, Casing},
};
use serde::Serialize;

///
/// FieldList
///

#[derive(Clone, Debug, Serialize)]
pub struct FieldList {
    pub fields: &'static [Field],
}

impl FieldList {
    // get
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }
}

impl ValidateNode for FieldList {}

impl VisitableNode for FieldList {
    fn drive<V: Visitor>(&self, v: &mut V) {
        for node in self.fields {
            node.accept(v);
        }
    }
}

///
/// Field
///

#[derive(Clone, Debug, Serialize)]
pub struct Field {
    pub name: &'static str,
    pub value: Value,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Arg>,
}

impl ValidateNode for Field {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();
        let ident = self.name;

        // idents
        errs.add_result(validate_ident(self.name));

        // snake case
        if !self.name.is_case(Case::Snake) {
            errs.add(format!("field name '{}' must be in snake_case", self.name));
        }

        // relation naming
        if self.value.item.is_relation() {
            match self.value.cardinality {
                Cardinality::One | Cardinality::Opt if !ident.ends_with("id") => {
                    errs.add(format!(
                        "one or optional relationship '{ident}' should end with 'id'"
                    ));
                }
                Cardinality::Many if !ident.ends_with("ids") => {
                    errs.add(format!("many relationship '{ident}' should end with 'ids'"));
                }

                _ => {}
            }
        }

        errs.result()
    }
}

impl VisitableNode for Field {
    fn route_key(&self) -> String {
        self.name.to_string()
    }

    fn drive<V: Visitor>(&self, v: &mut V) {
        self.value.accept(v);
        if let Some(node) = &self.default {
            node.accept(v);
        }
    }
}
