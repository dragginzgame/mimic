use crate::{
    schema::{
        build::validate::validate_ident,
        node::{Arg, ValidateNode, Value, VisitableNode},
        visit::Visitor,
    },
    types::{Cardinality, ErrorVec, SortDirection},
    utils::case::{Case, Casing},
};
use serde::{Deserialize, Serialize};

///
/// FieldList
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldList {
    pub fields: Vec<Field>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<FieldOrder>,
}

impl FieldList {
    // get_field
    #[must_use]
    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }
}

impl ValidateNode for FieldList {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // order
        for rule in &self.order {
            if self.get_field(&rule.field).is_none() {
                errs.add(format!("field '{}' not found", rule.field));
            }
        }

        errs.result()
    }
}

impl VisitableNode for FieldList {
    fn drive<V: Visitor>(&self, v: &mut V) {
        for node in &self.fields {
            node.accept(v);
        }
        for node in &self.order {
            node.accept(v);
        }
    }
}

///
/// Field
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub value: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Arg>,
}

impl ValidateNode for Field {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();
        let ident = self.name.clone();

        // idents
        errs.add_result(validate_ident(&self.name));

        // snake case
        if !self.name.is_case(Case::Snake) {
            errs.add(format!("field name '{}' must be in snake_case", self.name));
        }

        // check for relations with confusing idents
        if self.value.item.is_relation() {
            let one_suffix = ident.ends_with("id");
            let many_suffix = ident.ends_with("ids");

            match self.value.cardinality {
                Cardinality::Many if !many_suffix => {
                    errs.add(format!("many relationship '{ident}' should end with 'ids'"));
                }
                Cardinality::One | Cardinality::Opt if !one_suffix => {
                    errs.add(format!(
                        "one or optional relationship '{ident}' should end with 'id'"
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
