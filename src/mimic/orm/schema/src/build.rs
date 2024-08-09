use crate::{
    node::{Schema, VisitableNode},
    visit::Validator,
    Error,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use types::ErrorTree;

///
/// BuildError
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum BuildError {
    #[snafu(display("validation failed: {errors}"))]
    Validation { errors: ErrorTree },
}

///
/// SCHEMA
///

static SCHEMA: LazyLock<RwLock<Schema>> = LazyLock::new(|| RwLock::new(Schema::new()));

// schema_write
pub fn schema_write() -> RwLockWriteGuard<'static, Schema> {
    SCHEMA.write().unwrap()
}

// schema_read
pub fn schema_read() -> RwLockReadGuard<'static, Schema> {
    SCHEMA.read().unwrap()
}

/// validate
pub fn validate() -> Result<(), Error> {
    // validate using the visitor
    let mut visitor = Validator::new();
    schema_read().accept(&mut visitor);

    // result
    visitor
        .errors()
        .result()
        .map_err(|errors| BuildError::Validation { errors })?;

    Ok(())
}
