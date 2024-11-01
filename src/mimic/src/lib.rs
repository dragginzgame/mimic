///
/// mimic
/// [for external use only, keep out of reach of children]
///
pub mod api;
pub mod build;
pub mod core;
pub mod db;
pub mod ic;
pub mod macros;
pub mod orm;
pub mod utils;

pub mod export {
    pub use ctor;
    pub use num_traits;
    pub use remain;
    pub use strum;
}

extern crate self as mimic;

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
            subnet::request::{Request, RequestKind, Response},
            Error, StartupHooks,
        },
        core::state::{
            AppCommand, AppState, AppStateManager, CanisterState, CanisterStateManager,
            SubnetIndex, SubnetIndexManager, User, UserIndex, UserIndexManager,
        },
        db::Db,
        ic::{caller, format_cycles, id},
        log, mimic_end, mimic_start,
        orm::traits::{EntityDyn, EntityFixture, Path},
        perf, Log,
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::std::cell::RefCell;
}

// init
// schema generation requires a function stub to work on OSX
pub const fn init() {
    crate::orm::base::init()
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
