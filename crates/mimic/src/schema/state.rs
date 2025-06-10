use crate::{
    Error, ThisError,
    schema::{SchemaError, node::Schema},
};
use icu::{Log, log};
use std::sync::Mutex;

///
/// StateError
///

#[derive(Debug, ThisError)]
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

// init_schema_json
pub fn init_schema_json(json: &str) -> Result<(), Error> {
    let schema = serde_json::from_str::<Schema>(json)
        .map_err(|e| StateError::SerdeJson(e.to_string()))
        .map_err(SchemaError::StateError)?;

    log!(Log::Info, "init_schema: hash {}", schema.hash);

    let mut guard = SCHEMA
        .lock()
        .map_err(|e| StateError::Mutex(e.to_string()))
        .map_err(SchemaError::StateError)?;

    if guard.is_some() {
        Err(SchemaError::StateError(StateError::AlreadyInitialized))?;
    }

    *guard = Some(schema);

    Ok(())
}

///
/// GET
///

// get_schema
pub(crate) fn get_schema() -> Result<Schema, StateError> {
    let guard = SCHEMA
        .lock()
        .map_err(|e| StateError::Mutex(e.to_string()))?;

    let schema = guard.as_ref().cloned().ok_or(StateError::NotInitialized)?;

    Ok(schema)
}
