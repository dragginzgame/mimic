use crate::{
    orm::schema::{
        build::validate::{is_reserved, validate_ident},
        node::{ValidateNode, Value, VisitableNode},
        types::{Cardinality, SortDirection},
        visit::Visitor,
    },
    types::ErrorVec,
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
}

impl ValidateNode for Field {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();
        let ident = self.name.clone();

        // idents
        errs.add_result(validate_ident(&self.name));

        // snake case
        let name_to_check = if self.name.starts_with('_') {
            &self.name[1..]
        } else if self.name.ends_with('_') {
            // reverse the check
            // as only a reserved word can end with a _
            let new_name = &self.name[0..&self.name.len() - 1];
            if is_reserved(new_name).is_ok() {
                errs.add("only reserved words can end with _");
            }

            new_name
        } else {
            &self.name
        };
        if !name_to_check.is_case(Case::Snake) {
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
    fn drive<V: Visitor>(&self, v: &mut V) {
        self.value.accept(v);
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
