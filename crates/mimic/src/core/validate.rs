use crate::{
    common::error::ErrorTree,
    core::{
        traits::Visitable,
        visit::{ValidateVisitor, perform_visit},
    },
};
use thiserror::Error as ThisError;

///
/// Validate
///

#[derive(Debug, ThisError)]
pub enum ValidateError {
    #[error("validation failed: {0}")]
    Validation(ErrorTree),
}

// validate
pub fn validate(node: &dyn Visitable) -> Result<(), ValidateError> {
    let mut visitor = ValidateVisitor::new();
    perform_visit(&mut visitor, node, "");

    visitor.errors.result().map_err(ValidateError::Validation)?;

    Ok(())
}
