use crate::{
    build::schema_read,
    node::{Entity, Selector, TypeValidator, ValidateNode, VisitableNode},
    types::Primitive,
    visit::Visitor,
};
use mimic_common::error::ErrorTree;
use serde::Serialize;
use std::ops::Not;

///
/// Item
///

#[derive(Clone, Debug, Serialize)]
pub struct Item {
    pub target: ItemTarget,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<&'static str>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<&'static str>,

    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub validators: &'static [TypeValidator],

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

        match &self.target {
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

            ItemTarget::Primitive(_) => {}
        }

        // relation
        if let Some(relation) = &self.relation {
            if self.indirect {
                errs.add("relations cannot be set to indirect");
            }

            // Step 1: Ensure the relation path exists and is an Entity
            match schema.get_node_as::<Entity>(relation) {
                Some(entity) => {
                    // Step 2: Get target of the relation entity (usually from its primary key field)
                    let primary_field = entity.get_pk_field();
                    let relation_target = &primary_field.value.item.target;

                    // Step 3: Compare to self.target()
                    if &self.target != relation_target {
                        errs.add(format!(
                            "relation target type mismatch: expected {:?}, found {:?}",
                            relation_target, self.target
                        ));
                    }
                }
                None => {
                    errs.add(format!("relation entity '{relation}' not found"));
                }
            }
        }

        // selector
        if let Some(selector) = &self.selector && schema.get_node_as::<Selector>(selector).is_none() {
            errs.add(format!("selector path '{selector}' not found"));
        }

        errs.result()
    }
}

impl VisitableNode for Item {
    fn drive<V: Visitor>(&self, v: &mut V) {
        for node in self.validators {
            node.accept(v);
        }
    }
}

///
/// ItemTarget
///

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ItemTarget {
    Is(&'static str),
    Primitive(Primitive),
}
