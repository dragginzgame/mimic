use crate::{
    log,
    schema::{node::Schema, SchemaError},
    Error, Log, ThisError,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

///
/// StateError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum StateError {
    #[error("schema has already been initialized")]
    AlreadyInitialized,

    #[error("schema not yet initialized")]
    NotInitialized,

    #[error("mutex error: {0}")]
    Mutex(String),

    #[error("serde json error: {0}")]
    SerdeJson(String),
}

///
/// SCHEMA
///

static SCHEMA: Mutex<Option<Schema>> = Mutex::new(None);

///
/// INIT
///

// init_schema
fn init_schema(schema: Schema) -> Result<(), StateError> {
    log!(Log::Info, "init_schema: hash {}", schema.hash);

    let mut guard = SCHEMA
        .lock()
        .map_err(|e| StateError::Mutex(e.to_string()))?;

    if guard.is_some() {
        return Err(StateError::AlreadyInitialized);
    }

    *guard = Some(schema);

    Ok(())
}

// init_schema_json
pub fn init_schema_json(schema_json: &str) -> Result<(), Error> {
    let schema = serde_json::from_str::<Schema>(schema_json)
        .map_err(|e| StateError::SerdeJson(e.to_string()))
        .map_err(SchemaError::StateError)?;

    init_schema(schema).map_err(SchemaError::StateError)?;

    Ok(())
}

///
/// GET
///

// get_schema
pub fn get_schema() -> Result<Schema, Error> {
    let guard = SCHEMA
        .lock()
        .map_err(|e| StateError::Mutex(e.to_string()))
        .map_err(SchemaError::StateError)?;

    let schema = guard
        .as_ref()
        .cloned()
        .ok_or(StateError::NotInitialized)
        .map_err(SchemaError::StateError)?;

    Ok(schema)
}
