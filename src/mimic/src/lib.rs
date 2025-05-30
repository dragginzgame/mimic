///
/// mimic
/// [for external use only, keep out of reach of children]
///
pub mod base;
pub mod db;
pub mod interface;
pub mod macros;
pub mod query;
pub mod schema;
pub mod traits;
pub mod types;
pub mod utils;
pub mod visit;

// makes it easier to use internally
pub mod ic {
    pub use icu::ic::*;
}

pub mod export {
    pub use ctor;
    pub use derive_more;
    pub use num_traits;
    pub use remain;
}

extern crate self as mimic;

///
/// MIMIC PRELUDE
/// using _ brings traits into scope and avoids name conflicts
///

pub mod prelude_actor {
    pub use crate::{
        base::types::Ulid,
        mimic_start,
        query::traits::{LoadCollectionTrait as _, LoadQueryBuilderTrait as _},
    };
}

pub mod prelude {
    pub use crate::{
        base::types::{
            Blob, Bool, Decimal, Float32, Float64, Int, Int8, Int16, Int32, Int64, Int128, Nat,
            Nat8, Nat16, Nat32, Nat64, Nat128, Principal, Relation, RelationSet, Text, Ulid,
        },
        traits::{
            EntityDyn, EntityFixture, EntityId as _, Inner as _, NumCast, Orderable, Ordering,
            Path, Searchable, Selector as _, SortKeyValue as _, Validate as _, ValidateCustom,
            ValidatorBytes, ValidatorNumber, ValidatorString, Visitable,
        },
        types::{ErrorTree, FixtureList},
    };
    pub use ::candid::CandidType;
    pub use ::mimic_design::*;
}

use crate::types::ErrorTree;
use candid::CandidType;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error as ThisError;
use traits::Visitable;
use visit::{ValidateVisitor, perform_visit};

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum Error {
    #[error(transparent)]
    DbError(#[from] db::DbError),

    #[error(transparent)]
    InterfaceError(#[from] interface::InterfaceError),

    #[error(transparent)]
    QueryError(#[from] query::QueryError),

    #[error(transparent)]
    SchemaError(#[from] schema::SchemaError),

    #[error(transparent)]
    SerializeError(#[from] SerializeError),

    #[error(transparent)]
    ValidationError(#[from] ValidationError),
}

///
/// ValidationError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum ValidationError {
    #[error("validation failed: {0}")]
    Validation(ErrorTree),
}

// validate
pub fn validate(node: &dyn Visitable) -> Result<(), ValidationError> {
    let mut visitor = ValidateVisitor::new();
    perform_visit(&mut visitor, node, "");

    visitor
        .errors
        .result()
        .map_err(ValidationError::Validation)?;

    Ok(())
}

///
/// SerializeError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum SerializeError {
    #[error(transparent)]
    SerializeError(#[from] icu::serialize::SerializeError),
}

// serialize
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, SerializeError>
where
    T: Serialize,
{
    icu::serialize::serialize(ty).map_err(SerializeError::from)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, SerializeError>
where
    T: DeserializeOwned,
{
    icu::serialize::deserialize(bytes).map_err(SerializeError::from)
}
