pub mod auth;
pub mod entity;

pub use auth::AuthService;

use candid::CandidType;
use schema::node::Schema;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::Mutex;

///
/// SCHEMA
///

static SCHEMA: Mutex<Option<Schema>> = Mutex::new(None);

// get_schema
pub fn get_schema() -> Result<Schema, Error> {
    let guard = SCHEMA
        .lock()
        .map_err(|e| Error::Mutex { msg: e.to_string() })?;

    match *guard {
        Some(ref schema) => Ok(schema.clone()),
        None => Err(Error::NotInitialized),
    }
}

// init_schema
fn init_schema(schema: Schema) -> Result<(), Error> {
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
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("config has already been initialized"))]
    AlreadyInitialized,

    #[snafu(display("config not yet initialized"))]
    NotInitialized,

    #[snafu(display("mutex error: {msg}"))]
    Mutex { msg: String },

    #[snafu(display("serde json error: {msg}"))]
    SerdeJson { msg: String },
}