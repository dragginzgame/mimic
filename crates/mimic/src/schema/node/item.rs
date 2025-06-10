use crate::{
    schema::{
        build::schema_read,
        node::{Entity, Selector, TypeValidator, ValidateNode, VisitableNode},
        types::PrimitiveType,
        visit::Visitor,
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
    pub target: ItemTarget,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validators: Vec<TypeValidator>,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub indirect: bool,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub todo: bool,
}

impl Item {
    #[must_use]
    pub const fn is_relation(&self) -> bool {
        matches!(self.target, ItemTarget::Relation(_))
    }
}

impl ValidateNode for Item {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();
        let schema = schema_read();

        match &self.target {
            ItemTarget::Relation(rel) => {
                if self.indirect {
                    errs.add("relations cannot be set to indirect");
                }

                // has to be an entity
                errs.add_result(schema.check_node_as::<Entity>(rel));
            }

            ItemTarget::Is(path) => {
                // cannot be an entity
                if schema.check_node_as::<Entity>(path).is_ok() {
                    errs.add("a non-relation Item cannot reference an Entity");
                }

                // todo
                if let Some(node) = schema.get_node(path) {
                    match node.get_type() {
                        Some(tnode) => {
                            if !self.todo && tnode.ty().todo {
                                errs.add(format!(
                                    "you must specify todo if targeting a todo flagged item ({path})",
                                ));
                            }
                        }
                        None => errs.add("node is not a valid type"),
                    }
                }
            }

            ItemTarget::Prim(_) => {}
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

impl VisitableNode for Item {
    fn drive<V: Visitor>(&self, v: &mut V) {
        for node in &self.validators {
            node.accept(v);
        }
    }
}

///
/// ItemTarget
///

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemTarget {
    Is(String),
    Relation(String),
    Prim(PrimitiveType),
}
