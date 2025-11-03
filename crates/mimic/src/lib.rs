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
pub mod obs;
pub mod types;

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

pub mod export {
    pub use canic;
    pub use ctor;
    pub use derive_more;
    pub use num_traits;
    pub use remain;
}

extern crate self as mimic;

///
/// CRATE
///

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

///
/// CONSTANTS
///

pub const MAX_INDEX_FIELDS: usize = 4;

///
/// MIMIC PRELUDE
/// using _ brings traits into scope and avoids name conflicts
///

pub mod prelude {
    pub use crate::{
        core::{
            Key, Value,
            traits::{
                CreateView as _, EntityKind as _, FilterView as _, Inner as _, Path as _,
                UpdateView as _, View as _,
            },
            view::{Create, Filter, Update, View},
        },
        db,
        db::{
            executor::SaveExecutor,
            query::{
                self, FilterDsl, FilterExpr, FilterExt as _, LimitExpr, LimitExt as _, SortExpr,
                SortExt as _,
            },
            response::Response,
        },
        mimic_start,
        types::{Decimal, Ulid},
    };
    pub use candid::CandidType;
    pub use serde::{Deserialize, Serialize};
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
    // third party
    #[error("{0}")]
    CanicError(String),

    #[error("{0}")]
    CoreError(String),

    #[error("{0}")]
    DbError(String),

    #[error("{0}")]
    InterfaceError(String),
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

from_to_string!(canic::Error, CanicError);
from_to_string!(core::CoreError, CoreError);
from_to_string!(db::DbError, DbError);
from_to_string!(interface::InterfaceError, InterfaceError);
