//
// mimic
//
// [for external use only, keep out of reach of children]
//
pub mod core;
pub mod db;
pub mod design;
pub mod interface;
pub mod macros;

///
/// MIMIC CRATE EXPORTS
///

pub mod common {
    pub use mimic_common::*;
}

pub mod build {
    pub use mimic_build::*;
}

pub mod schema {
    pub use mimic_schema::*;
}

///
/// RE-EXPORTS
///

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
        core::{
            db::{EntityKey, EntityKeys},
            traits::{EntityFixture as _, EntityKind as _, Inner as _},
            types::Ulid,
            value::IndexValue,
        },
        db,
        db::{
            executor::SaveExecutor,
            query::{Cmp, SortDirection},
            response::LoadCollection,
            service::EntityService,
        },
        mimic_start,
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
    DataError(String),

    #[error("{0}")]
    InterfaceError(String),

    #[error("{0}")]
    SerializeError(String),

    #[error("{0}")]
    ValidateError(String),
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

from_to_string!(db::DataError, DataError);
from_to_string!(interface::InterfaceError, InterfaceError);

from_to_string!(core::serialize::SerializeError, SerializeError);
from_to_string!(core::validate::ValidateError, ValidateError);
