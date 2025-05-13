use crate::Error;
use mimic::schema::build::get_schema;
use serde::Serialize;
use thiserror::Error as ThisError;

///
/// SchemaError
///

#[derive(Debug, Serialize, ThisError)]
pub enum SchemaError {
    #[error("serde json error: {0}")]
    SerdeJson(String),
}

// get_schema_json
// to get the built schema via an executable
pub fn get_schema_json() -> Result<String, Error> {
    let schema = get_schema()?;

    let json = serde_json::to_string(&*schema)
        .map_err(|e| SchemaError::SerdeJson(e.to_string()))
        .map_err(Error::SchemaError)?;

    Ok(json)
}
