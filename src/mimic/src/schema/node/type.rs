use crate::{
    schema::{
        build::schema_read,
        node::{Args, ValidateNode, Validator, VisitableNode, Visitor},
    },
    types::ErrorVec,
};
use serde::{Deserialize, Serialize};
use std::ops::Not;

///
/// Type
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Type {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validators: Vec<TypeValidator>,

    #[serde(default, skip_serializing_if = "Not::not")]
    pub todo: bool,
}

impl Type {
    #[must_use]
    pub fn skip_serializing(&self) -> bool {
        self.validators.is_empty() && !self.todo
    }
}

impl ValidateNode for Type {}

impl VisitableNode for Type {
    fn drive<V: Visitor>(&self, v: &mut V) {
        for node in &self.validators {
            node.accept(v);
        }
    }
}

///
/// TypeValidator
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TypeValidator {
    pub path: String,

    #[serde(default, skip_serializing_if = "Args::is_empty")]
    pub args: Args,
}

impl ValidateNode for TypeValidator {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // check path
        let res = schema_read().check_node_as::<Validator>(&self.path);
        if let Err(e) = res {
            errs.add(e.to_string());
        }

        errs.result()
    }
}

impl VisitableNode for TypeValidator {}
