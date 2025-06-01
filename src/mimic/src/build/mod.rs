pub mod actor;
pub mod macros;

use crate::{Error, schema::build::get_schema};
use thiserror::Error as ThisError;

///
/// BuildError
///

#[derive(Debug, ThisError)]
pub enum BuildError {
    #[error("canister path not found: {0}")]
    CanisterNotFound(String),

    #[error("cannot deserialize schema: {0}")]
    SchemaInvalid(String),
}

// get_schema_json
// to get the built schema via an executable
pub fn get_schema_json() -> Result<String, Error> {
    let schema = get_schema()?;

    let json =
        serde_json::to_string(&*schema).map_err(|e| BuildError::SchemaInvalid(e.to_string()))?;

    Ok(json)
}
