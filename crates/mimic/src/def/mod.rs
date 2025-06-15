pub mod traits;
pub mod types;
pub mod visit;

use crate::{
    def::{
        traits::Visitable,
        visit::{ValidateVisitor, perform_visit},
    },
    error::ErrorTree,
};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use thiserror::Error as ThisError;

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

///
/// EntityValues
///

#[derive(Debug)]
pub struct EntityValues {
    map: HashMap<&'static str, Option<String>>,
}

impl EntityValues {
    /// Checks if all given fields are present
    #[must_use]
    pub fn has_all(&self, fields: &[&'static str]) -> bool {
        fields.iter().all(|f| self.map.contains_key(f))
    }

    /// Access a field directly
    #[must_use]
    pub fn get(&self, field: &str) -> Option<&Option<String>> {
        self.map.get(field)
    }
}

impl From<HashMap<&'static str, Option<String>>> for EntityValues {
    fn from(map: HashMap<&'static str, Option<String>>) -> Self {
        EntityValues { map }
    }
}
