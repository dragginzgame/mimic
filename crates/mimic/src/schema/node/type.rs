use crate::{
    schema::{
        build::schema_read,
        node::{Args, ValidateNode, Validator, VisitableNode, Visitor},
    },
    types::ErrorTree,
};
use serde::Serialize;
use std::ops::Not;

///
/// Type
///

#[derive(Clone, Debug, Serialize)]
pub struct Type {
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub validators: &'static [TypeValidator],

    #[serde(skip_serializing_if = "Not::not")]
    pub todo: bool,
}

impl ValidateNode for Type {}

impl VisitableNode for Type {
    fn drive<V: Visitor>(&self, v: &mut V) {
        for node in self.validators {
            node.accept(v);
        }
    }
}

///
/// TypeValidator
///

#[derive(Clone, Debug, Serialize)]
pub struct TypeValidator {
    pub path: &'static str,
    pub args: Args,
}

impl ValidateNode for TypeValidator {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();

        // check path
        let res = schema_read().check_node_as::<Validator>(self.path);
        if let Err(e) = res {
            errs.add(e.to_string());
        }

        errs.result()
    }
}

impl VisitableNode for TypeValidator {}
