use crate::{
    build::schema,
    node::{Args, Sanitizer, ValidateNode, Validator, VisitableNode},
};
use serde::{Deserialize, Serialize};
use types::ErrorVec;

///
/// TypeSanitizer
///

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TypeSanitizer {
    pub path: String,

    #[serde(default, skip_serializing_if = "Args::is_empty")]
    pub args: Args,
}

impl ValidateNode for TypeSanitizer {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // check path
        let res = schema().check_node::<Sanitizer>(&self.path);
        if let Err(e) = res {
            errs.add(e.to_string());
        }

        errs.result()
    }
}

impl VisitableNode for TypeSanitizer {}

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
        let res = schema().check_node::<Validator>(&self.path);
        if let Err(e) = res {
            errs.add(e.to_string());
        }

        errs.result()
    }
}

impl VisitableNode for TypeValidator {}
