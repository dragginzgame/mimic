pub mod macros;

///
/// mimic
/// [for external use only]
///
pub use api;
pub use build;
pub use config;
pub use ic;
pub use types;

pub mod core {
    pub use core_schema as schema;
    pub use core_state as state;
    pub use core_wasm as wasm;
}

pub mod db {
    pub use db::*;
    pub use db_query as query;
}

pub mod lib {
    pub use lib_case as case;
    pub use lib_cbor as cbor;
    pub use lib_rand as rand;
    pub use lib_time as time;
}

pub mod orm {
    pub mod prelude {
        pub use ::candid::{CandidType, Principal};
        pub use ::ic::structures::storable::Bound;
        pub use ::lib_case::{Case, Casing};
        pub use ::num_traits::{NumCast, ToPrimitive};
        pub use ::orm::{
            collections::HashSet,
            traits::{
                EntityDynamic, EntityFixture, EnumHash, EnumValue, Filterable, Inner, Orderable,
                Path, PrimaryKey, Sanitize, Storable, Validate, Visitable,
            },
        };
        pub use ::orm_macros::*;
        pub use ::serde::{Deserialize, Serialize};
        pub use ::snafu::Snafu;
        pub use ::std::{cmp::Ordering, fmt::Display};
        pub use ::types::ErrorVec;
    }

    pub use orm::*;
    pub use orm_macros as macros;
    pub use orm_schema as schema;
}

pub mod schema {
    pub use core_schema::{get_schema, Schema};
}

pub mod export {
    pub use ctor;
    pub use num_traits;
    pub use remain;
    pub use strum;
}

///
/// MIMIC PRELUDE
///
/// NOTE: Do not put the candid macros (query, update etc.) directly within this prelude as the endpoints
/// will fail to be registered with the export_candid! macro
///

pub mod prelude {
    pub use crate::{
        api::{
            auth::{guard, Guard},
            request::{Request, RequestKind, Response},
        },
        core::state::{
            AppCommand, AppState, AppStateManager, CanisterState, CanisterStateManager,
            SubnetIndex, SubnetIndexManager, User, UserIndex, UserIndexManager,
        },
        db::query as db_query,
        ic::{caller, format_cycles, id, log, Log},
        mimic_end, mimic_start,
        orm::traits::{EntityFixture, Path},
        perf,
        types::Ulid,
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::std::cell::RefCell;
}

///
/// ::mimic module code
///
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// ERROR
///
/// consolidates all the different crate errors into one place
///

pub const ERROR_API: u8 = 101;
pub const ERROR_CONFIG: u8 = 102;
pub const ERROR_DB: u8 = 103;
pub const ERROR_QUERY: u8 = 104;
pub const ERROR_CORE_SCHEMA: u8 = 105;
pub const ERROR_CORE_STATE: u8 = 106;
pub const ERROR_CORE_WASM: u8 = 107;

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct Error(u8, String);

impl Error {
    #[must_use]
    pub fn new(code: u8, text: String) -> Self {
        Self(code, text)
    }
}

impl From<api::Error> for Error {
    fn from(error: api::Error) -> Self {
        Self(ERROR_API, error.to_string())
    }
}

impl From<config::Error> for Error {
    fn from(error: config::Error) -> Self {
        Self(ERROR_CONFIG, error.to_string())
    }
}

impl From<db::Error> for Error {
    fn from(error: db::Error) -> Self {
        Self(ERROR_DB, error.to_string())
    }
}

impl From<db::query::Error> for Error {
    fn from(error: db::query::Error) -> Self {
        Self(ERROR_QUERY, error.to_string())
    }
}

impl From<core_schema::Error> for Error {
    fn from(error: core_schema::Error) -> Self {
        Self(ERROR_CORE_SCHEMA, error.to_string())
    }
}

impl From<core_state::Error> for Error {
    fn from(error: core_state::Error) -> Self {
        Self(ERROR_CORE_STATE, error.to_string())
    }
}

impl From<core_wasm::Error> for Error {
    fn from(error: core_wasm::Error) -> Self {
        Self(ERROR_CORE_WASM, error.to_string())
    }
}
