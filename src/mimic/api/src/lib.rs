pub mod auth;
pub mod canister;
pub mod cascade;
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

pub const ERROR_AUTH: u8 = 101;
pub const ERROR_CANISTER: u8 = 102;
pub const ERROR_CASCADE: u8 = 103;
pub const ERROR_CREATE: u8 = 104;
pub const ERROR_CRUD: u8 = 105;
pub const ERROR_MGMT: u8 = 106;
pub const ERROR_REQUEST: u8 = 107;
pub const ERROR_SCHEMA: u8 = 108;
pub const ERROR_SUBNET: u8 = 109;
pub const ERROR_UPGRADE: u8 = 11;

pub const ERROR_DB: u8 = 120;

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
// from
//

// ic call
impl From<(RejectionCode, String)> for Error {
    fn from(error: (RejectionCode, String)) -> Self {
        Self(ERROR_CALL_REJECTED, error.1.to_string())
    }
}

impl From<auth::Error> for Error {
    fn from(error: auth::Error) -> Self {
        Self(ERROR_AUTH, error.to_string())
    }
}

impl From<canister::Error> for Error {
    fn from(error: canister::Error) -> Self {
        Self(ERROR_CANISTER, error.to_string())
    }
}

impl From<cascade::Error> for Error {
    fn from(error: cascade::Error) -> Self {
        Self(ERROR_CASCADE, error.to_string())
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

impl From<state::Error> for Error {
    fn from(error: state::Error) -> Self {
        Self(ERROR_STATE, error.to_string())
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
