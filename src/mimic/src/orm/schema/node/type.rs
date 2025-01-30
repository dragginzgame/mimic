use crate::orm::{
    schema::{
        build::schema_read,
        node::{Args, ValidateNode, Validator, VisitableNode},
    },
    types::ErrorVec,
};
use serde::{Deserialize, Serialize};

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
        let res = schema_read().check_node::<Validator>(&self.path);
        if let Err(e) = res {
            errs.add(e.to_string());
        }

        errs.result()
    }
}

impl VisitableNode for TypeValidator {}
