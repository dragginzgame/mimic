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
pub mod query;
pub mod utils;

pub mod export {
    pub use ctor;
    pub use defer;
    pub use num_traits;
    pub use remain;
    pub use strum;
}

extern crate self as mimic;

use crate::{
    api::{
        auth::AuthError,
        ic::{
            call::CallError as IcCallError, canister::CanisterError as IcCanisterError,
            create::CreateError as IcCreateError, upgrade::UpgradeError as IcUpgradeError,
        },
        subnet::{
            cascade::CascadeError as SubnetCascadeError,
            request::RequestError as SubnetRequestError,
        },
    },
    core::{
        config::ConfigError as CoreConfigError,
        schema::SchemaError as CoreSchemaError,
        state::{
            AppStateError as CoreAppStateError, CanisterStateError as CoreCanisterStateError,
            ChildIndexError as CoreChildIndexError, SubnetIndexError as CoreSubnetIndexError,
            UserIndexError as CoreUserIndexError,
        },
        wasm::WasmError as CoreWasmError,
    },
    db::DbError,
    orm::OrmError,
    query::{
        DeleteError as QueryDeleteError, LoadError as QueryLoadError, QueryError,
        SaveError as QuerySaveError,
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{
    error::Error as StdError,
    fmt::{self, Display},
    future::Future,
};

///
/// ERROR
/// consolidates all the different crate errors into one place
///

// misc
pub const ERROR_DYN: u8 = 1;

// api
pub const ERROR_API_AUTH: u8 = 101;
pub const ERROR_API_CORE_SCHEMA: u8 = 110;
pub const ERROR_API_IC_CALL: u8 = 120;
pub const ERROR_API_IC_CANISTER: u8 = 121;
pub const ERROR_API_IC_CREATE: u8 = 122;
pub const ERROR_API_IC_MGMT: u8 = 123;
pub const ERROR_API_IC_UPGRADE: u8 = 124;
pub const ERROR_API_SUBNET_CASCADE: u8 = 130;
pub const ERROR_API_SUBNET_REQUEST: u8 = 131;

// core
pub const ERROR_CORE_CONFIG: u8 = 140;
pub const ERROR_CORE_SCHEMA: u8 = 141;
pub const ERROR_CORE_WASM: u8 = 142;
pub const ERROR_CORE_APP_STATE: u8 = 143;
pub const ERROR_CORE_CANISTER_STATE: u8 = 144;
pub const ERROR_CORE_CHILD_INDEX: u8 = 145;
pub const ERROR_CORE_SUBNET_INDEX: u8 = 146;
pub const ERROR_CORE_USER_INDEX: u8 = 147;

// orm
pub const ERROR_ORM: u8 = 150;

// db
pub const ERROR_DB: u8 = 160;

// query
pub const ERROR_DB_QUERY: u8 = 170;
pub const ERROR_DB_QUERY_LOAD: u8 = 171;
pub const ERROR_DB_QUERY_SAVE: u8 = 172;
pub const ERROR_DB_QUERY_DELETE: u8 = 173;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct Error(u8, String);

impl Error {
    #[must_use]
    pub const fn new(code: u8, text: String) -> Self {
        Self(code, text)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl StdError for Error {}

//
// dynamic errors
//

impl From<DynError> for Error {
    fn from(error: DynError) -> Self {
        Self(ERROR_API_AUTH, error.to_string())
    }
}

//
// api
//

impl From<AuthError> for Error {
    fn from(error: AuthError) -> Self {
        Self(ERROR_API_AUTH, error.to_string())
    }
}

//
// api ic
//

impl From<IcCallError> for Error {
    fn from(error: IcCallError) -> Self {
        Self(ERROR_API_IC_CALL, error.to_string())
    }
}

impl From<IcCanisterError> for Error {
    fn from(error: IcCanisterError) -> Self {
        Self(ERROR_API_IC_CANISTER, error.to_string())
    }
}

impl From<IcCreateError> for Error {
    fn from(error: IcCreateError) -> Self {
        Self(ERROR_API_IC_CREATE, error.to_string())
    }
}

impl From<IcUpgradeError> for Error {
    fn from(error: IcUpgradeError) -> Self {
        Self(ERROR_API_IC_UPGRADE, error.to_string())
    }
}

//
// api subnet
//

impl From<SubnetCascadeError> for Error {
    fn from(error: SubnetCascadeError) -> Self {
        Self(ERROR_API_SUBNET_CASCADE, error.to_string())
    }
}

impl From<SubnetRequestError> for Error {
    fn from(error: SubnetRequestError) -> Self {
        Self(ERROR_API_SUBNET_REQUEST, error.to_string())
    }
}

//
// core
//

impl From<CoreConfigError> for Error {
    fn from(error: CoreConfigError) -> Self {
        Self(ERROR_CORE_CONFIG, error.to_string())
    }
}

impl From<CoreSchemaError> for Error {
    fn from(error: CoreSchemaError) -> Self {
        Self(ERROR_CORE_SCHEMA, error.to_string())
    }
}

impl From<CoreAppStateError> for Error {
    fn from(error: CoreAppStateError) -> Self {
        Self(ERROR_CORE_APP_STATE, error.to_string())
    }
}

impl From<CoreCanisterStateError> for Error {
    fn from(error: CoreCanisterStateError) -> Self {
        Self(ERROR_CORE_CANISTER_STATE, error.to_string())
    }
}

impl From<CoreChildIndexError> for Error {
    fn from(error: CoreChildIndexError) -> Self {
        Self(ERROR_CORE_CHILD_INDEX, error.to_string())
    }
}

impl From<CoreSubnetIndexError> for Error {
    fn from(error: CoreSubnetIndexError) -> Self {
        Self(ERROR_CORE_SUBNET_INDEX, error.to_string())
    }
}

impl From<CoreUserIndexError> for Error {
    fn from(error: CoreUserIndexError) -> Self {
        Self(ERROR_CORE_USER_INDEX, error.to_string())
    }
}

impl From<CoreWasmError> for Error {
    fn from(error: CoreWasmError) -> Self {
        Self(ERROR_CORE_WASM, error.to_string())
    }
}

//
// orm
//

impl From<OrmError> for Error {
    fn from(error: OrmError) -> Self {
        Self(ERROR_ORM, error.to_string())
    }
}

//
// db
//

impl From<DbError> for Error {
    fn from(error: DbError) -> Self {
        Self(ERROR_DB, error.to_string())
    }
}

//
// query
//

impl From<QueryError> for Error {
    fn from(error: crate::query::QueryError) -> Self {
        Self(ERROR_DB_QUERY, error.to_string())
    }
}

impl From<QueryLoadError> for Error {
    fn from(error: QueryLoadError) -> Self {
        Self(ERROR_DB_QUERY_LOAD, error.to_string())
    }
}

impl From<QuerySaveError> for Error {
    fn from(error: QuerySaveError) -> Self {
        Self(ERROR_DB_QUERY_SAVE, error.to_string())
    }
}

impl From<QueryDeleteError> for Error {
    fn from(error: QueryDeleteError) -> Self {
        Self(ERROR_DB_QUERY_DELETE, error.to_string())
    }
}

///
/// DynError
///

pub type DynError = Box<dyn StdError + Send + Sync>;

///
/// MIMIC PRELUDE
///
/// NOTE: Do not put the candid macros (query, update etc.) directly within this prelude as the endpoints
/// will fail to be registered with the export_candid! macro
///

pub mod prelude {
    pub use crate::{
        api::{
            auth::{allow_any, Auth},
            guard::{guard_query, guard_update},
            subnet::request::{Request, RequestKind, Response},
        },
        core::state::{
            AppCommand, AppState, AppStateManager, CanisterState, CanisterStateManager,
            SubnetIndex, SubnetIndexManager, User, UserIndex, UserIndexManager,
        },
        db::Db,
        ic::{caller, format_cycles, id},
        log, mimic_end, mimic_start,
        orm::{
            base::types::Ulid,
            traits::{EntityDyn, EntityFixture, NumFromPrimitive, NumToPrimitive, Path, Validator},
        },
        perf, Log, StartupHooks,
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::std::cell::RefCell;
}

// init
// schema generation requires a function stub to work on OSX
pub const fn init() {
    crate::orm::base::init();
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

///
/// StartupHooks
///

pub trait StartupHooks {
    // startup
    // on every startup regardless of installation mode
    fn startup() -> Result<(), DynError> {
        Ok(())
    }

    // init
    // custom code called after mimic init()
    fn init() -> Result<(), DynError> {
        Ok(())
    }

    // init_async
    // custom code called after mimic init_async()
    #[must_use]
    fn init_async() -> impl Future<Output = Result<(), DynError>> + Send {
        async { Ok(()) }
    }

    // pre_upgrade
    // called after pre_upgrade
    fn pre_upgrade() -> Result<(), DynError> {
        Ok(())
    }

    // post_upgrade
    // called after post_upgrade
    fn post_upgrade() -> Result<(), DynError> {
        Ok(())
    }
}
