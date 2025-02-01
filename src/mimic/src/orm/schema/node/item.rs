use crate::orm::{
    schema::{
        build::schema_read,
        node::{Entity, Selector, ValidateNode, VisitableNode},
    },
    types::ErrorVec,
};
use serde::{Deserialize, Serialize};
use std::ops::Not;

///
/// Item
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(default, skip_serializing_if = "Not::not")]
    pub indirect: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

impl Item {
    // is_relation
    #[must_use]
    pub const fn is_relation(&self) -> bool {
        self.relation.is_some()
    }
}

impl ValidateNode for Item {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();
        let schema = schema_read();

        // both
        if self.is.is_some() == self.relation.is_some() {
            errs.add("only one of is or relation should be set");
        }

        // is
        if let Some(path) = &self.is {
            match schema.try_get_node(path) {
                Ok(node) => match node.get_type() {
                    Some(_) => {}
                    None => errs.add("node is not a valid type"),
                },
                Err(e) => errs.add(e),
            }
        }

        // relation
        if let Some(path) = &self.relation {
            if self.indirect {
                errs.add("relations cannot be set to indirect");
            }

            errs.add_result(schema.check_node_as::<Entity>(path));
        }

        // selector
        if let Some(selector) = &self.selector {
            if schema.get_node_as::<Selector>(selector).is_none() {
                errs.add(format!("selector path '{selector}' not found"));
            }
        }

        errs.result()
    }
}

impl VisitableNode for Item {}
