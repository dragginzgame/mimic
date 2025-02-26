///
/// mimic
/// [for external use only, keep out of reach of children]
///
pub mod config;
pub mod db;
pub mod ic;
pub mod macros;
pub mod orm;
pub mod query;
pub mod schema;
pub mod types;
pub mod utils;

pub mod export {
    pub use defer;
    pub use derive_more;
    pub use num_traits;
    pub use remain;
}

extern crate self as mimic;

///
/// MIMIC PRELUDE
///

pub mod prelude {
    pub use crate::{
        Log,
        db::Store,
        ic::{
            api::{canister_self, msg_caller},
            format_cycles, init, query, update,
        },
        log, mimic_end, mimic_memory_manager, mimic_start,
        orm::{
            base::{
                self,
                types::{Ulid, UlidSet},
            },
            traits::{EntityDyn, EntityFixture, NumFromPrimitive, NumToPrimitive, Path, Validator},
        },
        perf,
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::std::cell::RefCell;
}

use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

// init
// schema generation requires a function stub to work on OSX
pub const fn init() {
    crate::orm::base::init();
}

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum Error {
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    DbError(#[from] db::DbError),

    #[error(transparent)]
    IcError(#[from] ic::IcError),

    #[error(transparent)]
    OrmError(#[from] orm::OrmError),

    #[error(transparent)]
    QueryError(#[from] query::QueryError),

    #[error(transparent)]
    SchemaError(#[from] schema::SchemaError),
}

///
/// Logging
///

pub enum Log {
    Ok,
    Perf,
    Info,
    Warn,
    Error,
}

#[macro_export]
macro_rules! log {
    // Match when only the format string is provided (no additional args)
    ($level:expr, $fmt:expr) => {{
        // Pass an empty set of arguments to @inner
        log!(@inner $level, $fmt,);
    }};

    // Match when additional arguments are provided
    ($level:expr, $fmt:expr, $($arg:tt)*) => {{
        log!(@inner $level, $fmt, $($arg)*);
    }};

    // Inner macro for actual logging logic to avoid code duplication
    (@inner $level:expr, $fmt:expr, $($arg:tt)*) => {{
        let level: Log = $level;
        let formatted_message = format!($fmt, $($arg)*);  // Apply formatting with args

        let msg = match level {
            Log::Ok => format!("\x1b[32mOK\x1b[0m: {}", formatted_message),
            Log::Perf => format!("\x1b[35mPERF\x1b[0m: {}", formatted_message),
            Log::Info => format!("\x1b[34mINFO\x1b[0m: {}", formatted_message),
            Log::Warn => format!("\x1b[33mWARN\x1b[0m: {}", formatted_message),
            Log::Error => format!("\x1b[31mERROR\x1b[0m: {}", formatted_message),
        };

        $crate::ic::println!("{}", msg);
    }};
}
