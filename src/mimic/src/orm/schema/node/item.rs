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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub indirect: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub todo: bool,
}

impl Item {
    // is_relation
    #[must_use]
    pub const fn is_relation(&self) -> bool {
        self.relation.is_some()
    }

    // get_path
    fn get_path(&self) -> String {
        match (&self.is, &self.relation) {
            (Some(is), None) => is.to_string(),
            (None, Some(relation)) => relation.to_string(),
            _ => panic!("error should have been caught in macros"),
        }
    }
}

impl ValidateNode for Item {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();
        let schema = schema_read();

        // Validate 'is' field
        if let Some(path) = &self.is {
            if let Err(e) = schema.try_get_node(path) {
                errs.add(e);
            }
        }

        // relation
        if let Some(path) = &self.relation {
            if self.indirect {
                errs.add("relations cannot be set to indirect");
            }
            errs.add_result(schema.check_node_as::<Entity>(path));
        }

        // type node (both is and relation)
        if let Some(node) = schema.get_node(&self.get_path()) {
            match node.get_type() {
                Some(tnode) => {
                    let other_todo = tnode.ty().todo;
                    if self.todo != other_todo {
                        let message = if self.todo {
                            "this item's target is not flagged todo"
                        } else {
                            "you must specify todo if targeting a todo flagged item"
                        };
                        errs.add(message);
                    }
                }
                None => errs.add("node is not a valid type"),
            }
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
