pub mod auth;
pub mod call;
pub mod canister;
pub mod cascade;
pub mod config;
pub mod create;
pub mod crud;
pub mod mgmt;
pub mod request;
pub mod schema;
pub mod state;
pub mod subnet;
pub mod upgrade;

// re-export
pub use defer::defer;

use candid::CandidType;
use ic::api::call::RejectionCode;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

///
/// ERROR
/// consolidates all the different crate errors into one place
///

pub const ERROR_INIT: u8 = 10;

pub const ERROR_CALL_REJECTED: u8 = 100;

// api modules
pub const ERROR_AUTH: u8 = 101;
pub const ERROR_CALL: u8 = 102;
pub const ERROR_CANISTER: u8 = 103;
pub const ERROR_CONFIG: u8 = 104;
pub const ERROR_CREATE: u8 = 105;
pub const ERROR_CRUD: u8 = 106;
pub const ERROR_MGMT: u8 = 107;
pub const ERROR_REQUEST: u8 = 108;
pub const ERROR_SCHEMA: u8 = 109;
pub const ERROR_SUBNET: u8 = 110;
pub const ERROR_UPGRADE: u8 = 111;

// other crates
pub const ERROR_CORE_STATE: u8 = 120;
pub const ERROR_DB: u8 = 121;
pub const ERROR_QUERY: u8 = 122;

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct Error(u8, String);

impl Error {
    #[must_use]
    pub fn init(text: String) -> Self {
        Self(ERROR_INIT, text)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

//
// ic call
//

impl From<(RejectionCode, String)> for Error {
    fn from(error: (RejectionCode, String)) -> Self {
        Self(ERROR_CALL_REJECTED, error.1.to_string())
    }
}

//
// api modules
//

impl From<auth::Error> for Error {
    fn from(error: auth::Error) -> Self {
        Self(ERROR_AUTH, error.to_string())
    }
}

impl From<call::Error> for Error {
    fn from(error: call::Error) -> Self {
        Self(ERROR_CALL, error.to_string())
    }
}

impl From<canister::Error> for Error {
    fn from(error: canister::Error) -> Self {
        Self(ERROR_CANISTER, error.to_string())
    }
}

impl From<config::Error> for Error {
    fn from(error: config::Error) -> Self {
        Self(ERROR_CONFIG, error.to_string())
    }
}

impl From<create::Error> for Error {
    fn from(error: create::Error) -> Self {
        Self(ERROR_CREATE, error.to_string())
    }
}

impl From<crud::Error> for Error {
    fn from(error: crud::Error) -> Self {
        Self(ERROR_CRUD, error.to_string())
    }
}

impl From<request::Error> for Error {
    fn from(error: request::Error) -> Self {
        Self(ERROR_REQUEST, error.to_string())
    }
}

impl From<schema::Error> for Error {
    fn from(error: schema::Error) -> Self {
        Self(ERROR_SCHEMA, error.to_string())
    }
}

impl From<subnet::Error> for Error {
    fn from(error: subnet::Error) -> Self {
        Self(ERROR_SUBNET, error.to_string())
    }
}

impl From<upgrade::Error> for Error {
    fn from(error: upgrade::Error) -> Self {
        Self(ERROR_UPGRADE, error.to_string())
    }
}

//
// other crates
//

impl From<core_state::Error> for Error {
    fn from(error: core_state::Error) -> Self {
        Self(ERROR_CORE_STATE, error.to_string())
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
