use crate::Error;
use mimic::{Error as MimicError, schema::build::get_schema};
use serde::Serialize;
use thiserror::Error as ThisError;

///
/// SchemaError
///

#[derive(Debug, Serialize, ThisError)]
pub enum SchemaError {
    #[error("serde json error: {0}")]
    SerdeJson(String),

    #[error(transparent)]
    MimicError(#[from] MimicError),
}

// get_schema_json
// to get the built schema via an executable
pub fn get_schema_json() -> Result<String, Error> {
    let schema = get_schema().map_err(SchemaError::MimicError)?;

    let json = serde_json::to_string(&*schema)
        .map_err(|e| SchemaError::SerdeJson(e.to_string()))
        .map_err(Error::SchemaError)?;

    Ok(json)
}
