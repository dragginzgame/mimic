pub mod auth;

pub use crate::orm::schema::node::Schema;
pub use auth::AuthService;

use crate::{log, Log};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::Mutex;

///
/// SchemaError
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum SchemaError {
    #[snafu(display("schema has already been initialized"))]
    AlreadyInitialized,

    #[snafu(display("schema not yet initialized"))]
    NotInitialized,

    #[snafu(display("mutex error: {msg}"))]
    Mutex { msg: String },

    #[snafu(display("serde json error: {msg}"))]
    SerdeJson { msg: String },
}

///
/// SCHEMA
///

static SCHEMA: Mutex<Option<Schema>> = Mutex::new(None);

///
/// INIT
///

// init_schema
fn init_schema(schema: Schema) -> Result<(), SchemaError> {
    log!(Log::Info, "init_schema: hash {}", schema.hash);

    let mut guard = SCHEMA
        .lock()
        .map_err(|e| SchemaError::Mutex { msg: e.to_string() })?;

    if guard.is_some() {
        Err(SchemaError::AlreadyInitialized)
    } else {
        *guard = Some(schema);

        Ok(())
    }
}

// init_schema_json
pub fn init_schema_json(schema_json: &str) -> Result<(), SchemaError> {
    let schema = serde_json::from_str::<Schema>(schema_json)
        .map_err(|e| SchemaError::SerdeJson { msg: e.to_string() })?;

    init_schema(schema)
}

///
/// GET
///

// get_schema
pub fn get_schema() -> Result<Schema, SchemaError> {
    let guard = SCHEMA
        .lock()
        .map_err(|e| SchemaError::Mutex { msg: e.to_string() })?;

    guard.as_ref().map_or(
        Err(SchemaError::NotInitialized),
        |schema| Ok(schema.clone()),
    )
}
