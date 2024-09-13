pub mod auth;
pub mod core;
pub mod crud;
pub mod ic;
pub mod subnet;

// re-export
pub use defer::defer;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

///
/// ERROR
/// consolidates all the different crate errors into one place
///

pub const ERROR_INIT: u8 = 10;

pub const ERROR_AUTH: u8 = 101;
pub const ERROR_CRUD: u8 = 102;
pub const ERROR_CORE_CONFIG: u8 = 110;
pub const ERROR_CORE_SCHEMA: u8 = 111;
pub const ERROR_CORE_STATE: u8 = 112;
pub const ERROR_IC_CALL: u8 = 120;
pub const ERROR_IC_CANISTER: u8 = 121;
pub const ERROR_IC_CREATE: u8 = 122;
pub const ERROR_IC_MGMT: u8 = 123;
pub const ERROR_IC_UPGRADE: u8 = 124;
pub const ERROR_SUBNET_CASCADE: u8 = 130;
pub const ERROR_SUBNET_REQUEST: u8 = 131;

// other crates
pub const ERROR_DB: u8 = 140;
pub const ERROR_QUERY: u8 = 141;

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct Error(u8, String);

impl Error {
    #[must_use]
    pub fn init(text: String) -> Self {
        Self(ERROR_INIT, text)
    }

    #[must_use]
    pub fn new(code: u8, text: String) -> Self {
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

impl From<auth::Error> for Error {
    fn from(error: auth::Error) -> Self {
        Self(ERROR_AUTH, error.to_string())
    }
}

impl From<crud::Error> for Error {
    fn from(error: crud::Error) -> Self {
        Self(ERROR_CRUD, error.to_string())
    }
}

//
// core
//

impl From<core::config::Error> for Error {
    fn from(error: core::config::Error) -> Self {
        Self(ERROR_CORE_CONFIG, error.to_string())
    }
}

impl From<core::schema::Error> for Error {
    fn from(error: core::schema::Error) -> Self {
        Self(ERROR_CORE_SCHEMA, error.to_string())
    }
}

impl From<core::state::Error> for Error {
    fn from(error: core::state::Error) -> Self {
        Self(ERROR_CORE_STATE, error.to_string())
    }
}

//
// ic
//

impl From<ic::call::Error> for Error {
    fn from(error: ic::call::Error) -> Self {
        Self(ERROR_IC_CALL, error.to_string())
    }
}

impl From<ic::canister::Error> for Error {
    fn from(error: ic::canister::Error) -> Self {
        Self(ERROR_IC_CANISTER, error.to_string())
    }
}

impl From<ic::create::Error> for Error {
    fn from(error: ic::create::Error) -> Self {
        Self(ERROR_IC_CREATE, error.to_string())
    }
}

impl From<ic::upgrade::Error> for Error {
    fn from(error: ic::upgrade::Error) -> Self {
        Self(ERROR_IC_UPGRADE, error.to_string())
    }
}

//
// subnet
//

impl From<subnet::cascade::Error> for Error {
    fn from(error: subnet::cascade::Error) -> Self {
        Self(ERROR_SUBNET_CASCADE, error.to_string())
    }
}

impl From<subnet::request::Error> for Error {
    fn from(error: subnet::request::Error) -> Self {
        Self(ERROR_SUBNET_REQUEST, error.to_string())
    }
}

//
// other crates
// (fluent methods make it hard to return a compatible error)
//

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
