use crate::{
    common::error::ErrorTree,
    core::{
        traits::Visitable,
        visit::{ValidateVisitor, perform_visit},
    },
};
use thiserror::Error as ThisError;

///
/// ValidationError
///

#[derive(Debug, ThisError)]
pub enum ValidationError {
    #[error("validation failed: {0}")]
    ValidationFailed(ErrorTree),
}

// validate
pub fn validate(node: &dyn Visitable) -> Result<(), ValidationError> {
    let mut visitor = ValidateVisitor::new();
    perform_visit(&mut visitor, node, "");

    visitor
        .errors
        .result()
        .map_err(ValidationError::ValidationFailed)?;

    Ok(())
}
