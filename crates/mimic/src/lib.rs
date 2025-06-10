///
/// mimic
/// [for external use only, keep out of reach of children]
///
pub mod build;
pub mod data;
pub mod interface;
pub mod macros;
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

pub mod prelude {
    pub use crate::{
        data::executor::SaveExecutor,
        mimic_start, query_delete, query_load, query_load_dyn, query_save,
        schema::types::SortDirection,
        traits::{
            EntityFixture, EntityIdKind as _, EntityKind as _, FormatSortKey as _, Inner as _,
            NumCast, Orderable, Ordering, Path, Searchable, Validate as _, ValidateCustom,
            ValidatorBytes, ValidatorNumber, ValidatorString, Visitable,
        },
        types::{
            ErrorTree,
            prim::{Key, KeySet, Ulid},
        },
    };
    pub use ::candid::CandidType;
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
/// top level error should handle all sub-errors, but not expose the candid types
/// as that would be a lot for any project that uses mimic
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum Error {
    #[error("{0}")]
    BuildError(String),

    #[error("{0}")]
    DataError(String),

    #[error("{0}")]
    InterfaceError(String),

    #[error("{0}")]
    SchemaError(String),

    #[error("{0}")]
    SerializeError(String),

    #[error("{0}")]
    StorageError(String),

    #[error("{0}")]
    ValidationError(String),
}

macro_rules! from_to_string {
    ($from:ty, $variant:ident) => {
        impl From<$from> for Error {
            fn from(e: $from) -> Self {
                Error::$variant(e.to_string())
            }
        }
    };
}

from_to_string!(build::BuildError, BuildError);
from_to_string!(data::DataError, DataError);
from_to_string!(interface::InterfaceError, InterfaceError);
from_to_string!(schema::SchemaError, SchemaError);
from_to_string!(SerializeError, SerializeError);
from_to_string!(ValidationError, ValidationError);

///
/// Validation
///

#[derive(Debug, ThisError)]
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
/// Serialize
///

#[derive(Debug, ThisError)]
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
