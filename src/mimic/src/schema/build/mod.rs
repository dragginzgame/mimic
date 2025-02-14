pub mod reserved;
pub mod validate;

use crate::{
    schema::{
        node::{Schema, VisitableNode},
        visit::Validator,
        SchemaError,
    },
    types::ErrorTree,
    Error as MimicError, ThisError,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    sync::{LazyLock, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

///
/// BuildError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum BuildError {
    #[error("validation failed: {0}")]
    Validation(ErrorTree),
}

///
/// Builder
/// hooks that can be registered in advance of building
///

pub struct Builder {
    pub reserved_prefixes: HashSet<&'static str>,
    pub reserved_words: HashSet<&'static str>,
}

impl Builder {
    pub fn add_reserved_prefixes(prefixes: &[&'static str]) {
        let mut builder = BUILDER.write().unwrap();

        builder.reserved_prefixes.extend(prefixes);
    }

    pub fn add_reserved_words(words: &[&'static str]) {
        let mut builder = BUILDER.write().unwrap();

        builder.reserved_words.extend(words);
    }
}

static BUILDER: LazyLock<RwLock<Builder>> = LazyLock::new(|| {
    RwLock::new(Builder {
        reserved_prefixes: HashSet::new(),
        reserved_words: reserved::WORDS.clone(),
    })
});

// schema_builder
// To interact with the singleton builder
pub fn schema_builder() -> RwLockReadGuard<'static, Builder> {
    BUILDER.read().unwrap()
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
pub fn get_schema() -> Result<RwLockReadGuard<'static, Schema>, MimicError> {
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
    let mut visitor = Validator::new();
    schema.accept(&mut visitor);
    visitor.errors().result()?;

    SCHEMA_VALIDATED.set(true).ok();

    Ok(())
}
