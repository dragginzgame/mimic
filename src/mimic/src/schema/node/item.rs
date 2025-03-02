use crate::{
    schema::{
        build::schema_read,
        node::{Entity, Selector, ValidateNode, VisitableNode},
    },
    types::ErrorTree,
};
use serde::{Deserialize, Serialize};
use std::ops::Not;

///
/// Item
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(default)]
    pub path: String,

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
    #[must_use]
    pub const fn is_relation(&self) -> bool {
        self.relation.is_some()
    }
}

impl ValidateNode for Item {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();
        let schema = schema_read();

        // Validate path
        if let Err(e) = schema.try_get_node(&self.path) {
            errs.add(e);
        }

        // relation
        if let Some(relation) = &self.relation {
            if self.indirect {
                errs.add("relations cannot be set to indirect");
            }
            errs.add_result(schema.check_node_as::<Entity>(relation));
        }

        // type node (both is and relation)
        if let Some(node) = schema.get_node(&self.path) {
            match node.get_type() {
                Some(tnode) => {
                    if !self.todo && tnode.ty().todo {
                        errs.add(format!(
                            "you must specify todo if {} targeting a todo flagged item",
                            &self.path
                        ));
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
