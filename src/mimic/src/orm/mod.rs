pub mod base;
pub mod traits;
pub mod visit;

use crate::{Error, ThisError, types::ErrorTree};
use candid::CandidType;
use icu::serialize::SerializeError;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use traits::Visitable;
use visit::{ValidateVisitor, perform_visit};

///
/// OrmError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum OrmError {
    // entity not found, used for auto-generated endpoints
    #[error("entity not found: {0}")]
    EntityNotFound(String),

    #[error("validation failed: {0}")]
    Validation(ErrorTree),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),
}

// validate
pub fn validate(node: &dyn Visitable) -> Result<(), Error> {
    let mut visitor = ValidateVisitor::new();
    perform_visit(&mut visitor, node, "");

    visitor.errors.result().map_err(OrmError::Validation)?;

    Ok(())
}

// serialize
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, OrmError>
where
    T: Serialize,
{
    icu::serialize::serialize(ty).map_err(OrmError::SerializeError)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, OrmError>
where
    T: DeserializeOwned,
{
    icu::serialize::deserialize(bytes).map_err(OrmError::SerializeError)
}
