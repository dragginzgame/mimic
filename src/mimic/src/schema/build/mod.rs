pub mod reserved;
pub mod validate;

use crate::{
    Error, ThisError,
    schema::{
        SchemaError,
        node::{Schema, VisitableNode},
        visit::ValidateVisitor,
    },
    types::ErrorTree,
};
use std::sync::{LazyLock, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

///
/// BuildError
///

#[derive(Debug, ThisError)]
pub enum BuildError {
    #[error("validation failed: {0:?}")]
    Validation(ErrorTree),
}

///
/// SCHEMA
/// the static data structure
///

static SCHEMA: LazyLock<RwLock<Schema>> = LazyLock::new(|| RwLock::new(Schema::new()));

static SCHEMA_VALIDATED: OnceLock<bool> = OnceLock::new();

// schema_write
pub fn schema_write() -> RwLockWriteGuard<'static, Schema> {
    SCHEMA.write().unwrap()
}

// schema_read
// just reads the schema directly without validation
pub(crate) fn schema_read() -> RwLockReadGuard<'static, Schema> {
    SCHEMA.read().unwrap()
}

// get_schema
// validate will only be done once
pub fn get_schema() -> Result<RwLockReadGuard<'static, Schema>, Error> {
    let schema = schema_read();
    validate(&schema)
        .map_err(BuildError::Validation)
        .map_err(SchemaError::BuildError)?;

    Ok(schema)
}

// validate
fn validate(schema: &Schema) -> Result<(), ErrorTree> {
    if *SCHEMA_VALIDATED.get_or_init(|| false) {
        return Ok(());
    }

    // validate
    let mut visitor = ValidateVisitor::new();
    schema.accept(&mut visitor);
    visitor.errors.result()?;

    SCHEMA_VALIDATED.set(true).ok();

    Ok(())
}
