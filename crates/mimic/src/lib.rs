///
/// mimic
/// [for external use only, keep out of reach of children]
///
pub mod build;
pub mod db;
pub mod def;
pub mod error;
pub mod interface;
pub mod macros;
pub mod schema;
pub mod types;
pub mod utils;

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
        db::{
            executor::SaveExecutor,
            response::{LoadCollection, LoadCollectionDyn},
            service::EntityService,
            types::SortDirection,
        },
        def::traits::{
            EntityFixture, EntityIdKind as _, EntityKind as _, Inner as _, NumCast as _, Path as _,
            Validate as _, ValidateCustom, ValidatorBytes as _, ValidatorNumber as _,
            ValidatorString as _, Visitable as _,
        },
        error::ErrorTree,
        mimic_start, query_delete, query_load, query_load_dyn, query_save,
        types::{Key, KeySet, Ulid},
    };
    pub use ::candid::CandidType;
}

use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// Error
///
/// top level error should handle all sub-errors, but not expose the candid types
/// as that would be a lot for any project that uses mimic
///

#[derive(CandidType, Debug, Deserialize, Serialize, ThisError)]
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
from_to_string!(db::DataError, DataError);
from_to_string!(interface::InterfaceError, InterfaceError);
from_to_string!(schema::SchemaError, SchemaError);
from_to_string!(def::SerializeError, SerializeError);
from_to_string!(def::ValidationError, ValidationError);
