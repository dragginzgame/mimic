pub mod auth;
pub mod entity_crud;

pub use auth::AuthService;
pub use orm_schema::node::Schema;

use lib_ic::{log, Log};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::Mutex;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("config has already been initialized"))]
    AlreadyInitialized,

    #[snafu(display("config not yet initialized"))]
    NotInitialized,

    #[snafu(display("mutex error: {msg}"))]
    Mutex { msg: String },

    #[snafu(display("serde json error: {msg}"))]
    SerdeJson { msg: String },

    #[snafu(transparent)]
    Entity { source: entity_crud::Error },
}

///
/// SCHEMA
///

static SCHEMA: Mutex<Option<Schema>> = Mutex::new(None);

///
/// INIT
///

// init_schema
fn init_schema(schema: Schema) -> Result<(), Error> {
    log!(Log::Info, "init_schema: hash {}", schema.hash);

    let mut guard = SCHEMA
        .lock()
        .map_err(|e| Error::Mutex { msg: e.to_string() })?;

    if guard.is_some() {
        Err(Error::AlreadyInitialized)
    } else {
        *guard = Some(schema);

        Ok(())
    }
}

// init_schema_json
pub fn init_schema_json(schema_json: &str) -> Result<(), Error> {
    let schema = serde_json::from_str::<Schema>(schema_json)
        .map_err(|e| Error::SerdeJson { msg: e.to_string() })?;

    init_schema(schema)
}

///
/// GET
///

// get_schema
pub fn get_schema() -> Result<Schema, Error> {
    let guard = SCHEMA
        .lock()
        .map_err(|e| Error::Mutex { msg: e.to_string() })?;

    guard
        .as_ref()
        .map_or(Err(Error::NotInitialized), |schema| Ok(schema.clone()))
}

// get_schema_json
pub fn get_schema_json() -> Result<String, Error> {
    let schema = get_schema()?;

    serde_json::to_string(&schema).map_err(|e| Error::SerdeJson { msg: e.to_string() })
}
