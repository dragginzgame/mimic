use crate::{
    schema::{
        build::validate::validate_ident,
        node::{Arg, ValidateNode, Value, VisitableNode},
        types::{Cardinality, SortDirection},
        visit::Visitor,
    },
    types::ErrorTree,
    utils::case::{Case, Casing},
};
use serde::{Deserialize, Serialize};

///
/// Field
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: Value,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Arg>,
}

impl ValidateNode for Field {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();
        let ident = self.name.clone();

        // idents
        errs.add_result(validate_ident(&self.name));

        // snake case
        if !self.name.is_case(Case::Snake) {
            errs.add(format!("field name '{}' must be in snake_case", self.name));
        }

        // relation naming
        if self.value.item.is_relation() {
            match self.value.cardinality {
                Cardinality::One | Cardinality::Opt if !ident.ends_with("rel") => {
                    errs.add(format!(
                        "one or optional relationship '{ident}' should end with 'rel'"
                    ));
                }
                Cardinality::Many if !ident.ends_with("rels") => {
                    errs.add(format!(
                        "many relationship '{ident}' should end with 'rels'"
                    ));
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

///
/// FieldOrder
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldOrder {
    pub field: String,
    pub direction: SortDirection,
}

impl ValidateNode for FieldOrder {}

impl VisitableNode for FieldOrder {}
