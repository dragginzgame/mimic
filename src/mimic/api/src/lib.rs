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

use candid::{
    decode_args, encode_args,
    utils::{ArgumentDecoder, ArgumentEncoder},
    CandidType, Principal,
};
use ic::{
    api::call::{call_raw, RejectionCode},
    log, Log,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    future::Future,
};

///
/// ERROR
/// consolidates all the different crate errors into one place
///

pub const ERROR_IC: u8 = 100;

pub const ERROR_AUTH: u8 = 101;
pub const ERROR_CANISTER: u8 = 102;
pub const ERROR_CREATE: u8 = 103;
pub const ERROR_CRUD: u8 = 104;
pub const ERROR_REQUEST: u8 = 105;
pub const ERROR_SCHEMA: u8 = 106;
pub const ERROR_SUBNET: u8 = 107;
pub const ERROR_UPGRADE: u8 = 108;

pub const ERROR_CONFIG: u8 = 109;
pub const ERROR_CORE_SCHEMA: u8 = 110;
pub const ERROR_CORE_STATE: u8 = 111;
pub const ERROR_CORE_WASM: u8 = 112;
pub const ERROR_DB: u8 = 113;
pub const ERROR_QUERY: u8 = 114;

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct Error(u8, String);

impl Error {
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
// ic
//

impl From<(RejectionCode, String)> for Error {
    fn from(error: (RejectionCode, String)) -> Self {
        Self(ERROR_IC, error.1)
    }
}

//
// api
//

impl From<auth::AuthError> for Error {
    fn from(error: auth::AuthError) -> Self {
        Self(ERROR_AUTH, error.to_string())
    }
}

impl From<canister::CanisterError> for Error {
    fn from(error: canister::CanisterError) -> Self {
        Self(ERROR_CANISTER, error.to_string())
    }
}

impl From<create::CreateError> for Error {
    fn from(error: create::CreateError) -> Self {
        Self(ERROR_CREATE, error.to_string())
    }
}

impl From<crud::CrudError> for Error {
    fn from(error: crud::CrudError) -> Self {
        Self(ERROR_CRUD, error.to_string())
    }
}

impl From<request::RequestError> for Error {
    fn from(error: request::RequestError) -> Self {
        Self(ERROR_REQUEST, error.to_string())
    }
}

impl From<schema::SchemaError> for Error {
    fn from(error: schema::SchemaError) -> Self {
        Self(ERROR_SCHEMA, error.to_string())
    }
}

impl From<subnet::SubnetError> for Error {
    fn from(error: subnet::SubnetError) -> Self {
        Self(ERROR_SUBNET, error.to_string())
    }
}

impl From<upgrade::UpgradeError> for Error {
    fn from(error: upgrade::UpgradeError) -> Self {
        Self(ERROR_UPGRADE, error.to_string())
    }
}

//
// crates
//

impl From<config::Error> for Error {
    fn from(error: config::Error) -> Self {
        Self(ERROR_CONFIG, error.to_string())
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

impl From<db::Error> for Error {
    fn from(error: db::Error) -> Self {
        Self(ERROR_DB, error.to_string())
    }
}

impl From<db_query::Error> for Error {
    fn from(error: db_query::Error) -> Self {
        Self(ERROR_QUERY, error.to_string())
    }
}

//
// call
// wrapping this because otherwise the error is a pain to handle
//

pub fn call<T: ArgumentEncoder, R: for<'a> ArgumentDecoder<'a>>(
    id: Principal,
    method: &str,
    args: T,
) -> impl Future<Output = Result<R, Error>> + Send + Sync {
    log!(Log::Info, "call: {method}@{id}");

    let args_raw = encode_args(args).expect("Failed to encode arguments.");
    let fut = call_raw(id, method, args_raw, 0);

    async {
        let bytes = fut.await?;
        decode_args(&bytes).map_err(decoder_error_to_reject::<R>)
    }
}

#[allow(clippy::needless_pass_by_value)]
fn decoder_error_to_reject<T>(err: candid::error::Error) -> Error {
    (
        RejectionCode::CanisterError,
        format!(
            "failed to decode canister response as {}: {}",
            std::any::type_name::<T>(),
            err
        ),
    )
        .into()
}
