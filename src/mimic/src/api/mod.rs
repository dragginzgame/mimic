pub mod auth;
pub mod core;
pub mod guard;
pub mod ic;
pub mod subnet;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    future::Future,
};

///
/// StartupHooks
///

pub trait StartupHooks {
    // startup
    // on every startup regardless of installation mode
    fn startup() -> Result<(), Error> {
        Ok(())
    }

    // init
    // custom code called after mimic init()
    fn init() -> Result<(), Error> {
        Ok(())
    }

    // init_async
    // custom code called after mimic init_async()
    #[must_use]
    fn init_async() -> impl Future<Output = Result<(), Error>> + Send {
        async { Ok(()) }
    }

    // pre_upgrade
    // called after pre_upgrade
    fn pre_upgrade() -> Result<(), Error> {
        Ok(())
    }

    // post_upgrade
    // called after post_upgrade
    fn post_upgrade() -> Result<(), Error> {
        Ok(())
    }
}

///
/// ERROR
/// consolidates all the different crate errors into one place
///

// misc
pub const ERROR_INIT: u8 = 10;

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
pub const ERROR_CORE_STATE: u8 = 142;
pub const ERROR_CORE_WASM: u8 = 143;

// orm
pub const ERROR_ORM: u8 = 150;

// db
pub const ERROR_DB: u8 = 160;

// query
pub const ERROR_DB_QUERY: u8 = 170;
pub const ERROR_DB_QUERY_LOAD: u8 = 171;
pub const ERROR_DB_QUERY_SAVE: u8 = 172;
pub const ERROR_DB_QUERY_DELETE: u8 = 173;

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct Error(u8, String);

impl Error {
    #[must_use]
    pub const fn init(text: String) -> Self {
        Self(ERROR_INIT, text)
    }

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

//
// api
//

impl From<auth::AuthError> for Error {
    fn from(error: auth::AuthError) -> Self {
        Self(ERROR_API_AUTH, error.to_string())
    }
}

//
// api core
//

impl From<core::schema::Error> for Error {
    fn from(error: core::schema::Error) -> Self {
        Self(ERROR_API_CORE_SCHEMA, error.to_string())
    }
}

//
// api ic
//

impl From<ic::call::Error> for Error {
    fn from(error: ic::call::Error) -> Self {
        Self(ERROR_API_IC_CALL, error.to_string())
    }
}

impl From<ic::canister::Error> for Error {
    fn from(error: ic::canister::Error) -> Self {
        Self(ERROR_API_IC_CANISTER, error.to_string())
    }
}

impl From<ic::create::Error> for Error {
    fn from(error: ic::create::Error) -> Self {
        Self(ERROR_API_IC_CREATE, error.to_string())
    }
}

impl From<ic::upgrade::Error> for Error {
    fn from(error: ic::upgrade::Error) -> Self {
        Self(ERROR_API_IC_UPGRADE, error.to_string())
    }
}

//
// api subnet
//

impl From<subnet::cascade::Error> for Error {
    fn from(error: subnet::cascade::Error) -> Self {
        Self(ERROR_API_SUBNET_CASCADE, error.to_string())
    }
}

impl From<subnet::request::Error> for Error {
    fn from(error: subnet::request::Error) -> Self {
        Self(ERROR_API_SUBNET_REQUEST, error.to_string())
    }
}

//
// core
//

impl From<crate::core::config::Error> for Error {
    fn from(error: crate::core::config::Error) -> Self {
        Self(ERROR_CORE_CONFIG, error.to_string())
    }
}

impl From<crate::core::schema::Error> for Error {
    fn from(error: crate::core::schema::Error) -> Self {
        Self(ERROR_CORE_SCHEMA, error.to_string())
    }
}

impl From<crate::core::state::app_state::Error> for Error {
    fn from(error: crate::core::state::app_state::Error) -> Self {
        Self(ERROR_CORE_STATE, error.to_string())
    }
}

impl From<crate::core::state::subnet_index::Error> for Error {
    fn from(error: crate::core::state::subnet_index::Error) -> Self {
        Self(ERROR_CORE_STATE, error.to_string())
    }
}

impl From<crate::core::state::user_index::Error> for Error {
    fn from(error: crate::core::state::user_index::Error) -> Self {
        Self(ERROR_CORE_STATE, error.to_string())
    }
}

impl From<crate::core::wasm::Error> for Error {
    fn from(error: crate::core::wasm::Error) -> Self {
        Self(ERROR_CORE_WASM, error.to_string())
    }
}

//
// orm
//

impl From<crate::orm::OrmError> for Error {
    fn from(error: crate::orm::OrmError) -> Self {
        Self(ERROR_ORM, error.to_string())
    }
}

//
// db
//

impl From<crate::db::DbError> for Error {
    fn from(error: crate::db::DbError) -> Self {
        Self(ERROR_DB, error.to_string())
    }
}

//
// query
//

impl From<crate::query::QueryError> for Error {
    fn from(error: crate::query::QueryError) -> Self {
        Self(ERROR_DB_QUERY, error.to_string())
    }
}

impl From<crate::query::LoadError> for Error {
    fn from(error: crate::query::LoadError) -> Self {
        Self(ERROR_DB_QUERY_LOAD, error.to_string())
    }
}

impl From<crate::query::SaveError> for Error {
    fn from(error: crate::query::SaveError) -> Self {
        Self(ERROR_DB_QUERY_SAVE, error.to_string())
    }
}

impl From<crate::query::DeleteError> for Error {
    fn from(error: crate::query::DeleteError) -> Self {
        Self(ERROR_DB_QUERY_DELETE, error.to_string())
    }
}
