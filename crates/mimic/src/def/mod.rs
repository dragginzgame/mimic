pub mod kind;
pub mod traits;
pub mod types;
pub mod visit;

use crate::error::ErrorTree;
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error as ThisError;
use traits::Visitable;
use visit::{ValidateVisitor, perform_visit};

///

///
/// Validation
///

#[derive(Debug, ThisError)]
pub enum ValidationError {
    #[error("validation failed: {0}")]
    Validation(ErrorTree),
}

// validate
pub fn validate(node: &dyn Visitable) -> Result<(), ValidationError> {
    let mut visitor = ValidateVisitor::new();
    perform_visit(&mut visitor, node, "");

    visitor
        .errors
        .result()
        .map_err(ValidationError::Validation)?;

    Ok(())
}

///
/// Serialize
///

#[derive(Debug, ThisError)]
pub enum SerializeError {
    #[error(transparent)]
    SerializeError(#[from] icu::serialize::SerializeError),
}

// serialize
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, SerializeError>
where
    T: Serialize,
{
    icu::serialize::serialize(ty).map_err(SerializeError::from)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, SerializeError>
where
    T: DeserializeOwned,
{
    icu::serialize::deserialize(bytes).map_err(SerializeError::from)
}
