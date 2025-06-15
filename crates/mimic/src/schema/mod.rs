pub mod build;
pub mod node;
pub mod types;
pub mod visit;

pub use build::get_schema;

use crate::{
    ThisError,
    schema::{build::BuildError, node::NodeError},
};
use std::collections::HashMap;

///
/// SchemaError
///

#[derive(Debug, ThisError)]
pub enum SchemaError {
    #[error(transparent)]
    BuildError(#[from] BuildError),

    #[error(transparent)]
    NodeError(#[from] NodeError),
}

///
/// EntityValues
///

pub struct EntityValues {
    map: HashMap<&'static str, Option<String>>,
}

impl EntityValues {
    /// Checks if all given fields are present
    pub fn has_all(&self, fields: &[&'static str]) -> bool {
        fields.iter().all(|f| self.map.contains_key(f))
    }

    /// Access a field directly
    pub fn get(&self, field: &str) -> Option<&Option<String>> {
        self.map.get(field)
    }
}
