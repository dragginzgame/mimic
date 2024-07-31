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

// schema
pub fn schema() -> RwLockReadGuard<'static, Schema> {
    SCHEMA.read().unwrap()
}

/// validate
pub fn validate() -> Result<(), Error> {
    // validate using the visitor
    let mut visitor = Validator::new();
    schema().accept(&mut visitor);

    // result
    visitor
        .errors()
        .result()
        .map_err(|errors| Error::Validation { errors })
}
