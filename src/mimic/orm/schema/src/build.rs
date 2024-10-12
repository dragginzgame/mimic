use crate::{
    node::{Schema, VisitableNode},
    visit::Validator,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use types::ErrorTree;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("serde json error: {msg}"))]
    SerdeJson { msg: String },

    #[snafu(display("validation failed: {errors}"))]
    Validation { errors: ErrorTree },
}

///
/// SCHEMA
///

static SCHEMA: LazyLock<RwLock<Schema>> = LazyLock::new(|| RwLock::new(Schema::new()));

// This flag tracks whether validation has been performed
static VALIDATION_DONE: LazyLock<RwLock<bool>> = LazyLock::new(|| RwLock::new(false));

// schema_write
pub fn schema_write() -> RwLockWriteGuard<'static, Schema> {
    SCHEMA.write().unwrap()
}

// schema_read
// just reads the schema directly without validation
pub(crate) fn schema_read() -> RwLockReadGuard<'static, Schema> {
    SCHEMA.read().unwrap()
}

/// schema
pub fn schema() -> Result<RwLockReadGuard<'static, Schema>, Error> {
    let schema = schema_read();

    // Check if validation has already been done
    let mut validation_done = VALIDATION_DONE.write().unwrap();
    if !*validation_done {
        validate(&schema)?;
        *validation_done = true;
    }

    Ok(schema)
}

// schema_json
// to get the built schema via an executable
pub fn schema_json() -> Result<String, Error> {
    let schema = schema()?;
    let json =
        serde_json::to_string(&*schema).map_err(|e| Error::SerdeJson { msg: e.to_string() })?;

    Ok(json)
}

// validate
fn validate(schema: &Schema) -> Result<(), Error> {
    let mut visitor = Validator::new();
    schema.accept(&mut visitor);
    ic::println!("schema.validate()");

    // result
    visitor
        .errors()
        .result()
        .map_err(|errors| Error::Validation { errors })?;

    Ok(())
}
