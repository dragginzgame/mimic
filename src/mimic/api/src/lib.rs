pub mod auth;
pub mod canister;
pub mod cascade;
pub mod create;
pub mod crud;
pub mod mgmt;
pub mod request;
pub mod schema;
pub mod state;
pub mod upgrade;

// re-export
pub use defer::defer;

use candid::{
    decode_args, encode_args,
    utils::{ArgumentDecoder, ArgumentEncoder},
    CandidType, Principal,
};
use lib_ic::api::call::{call_raw, RejectionCode};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::future::Future;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    ///
    /// api errors
    ///

    #[snafu(transparent)]
    Auth { source: auth::AuthError },

    #[snafu(transparent)]
    Canister { source: canister::CanisterError },

    #[snafu(transparent)]
    Create { source: create::CreateError },

    #[snafu(transparent)]
    Crud { source: crud::CrudError },

    #[snafu(transparent)]
    Request { source: request::RequestError },

    #[snafu(transparent)]
    Schema { source: schema::SchemaError },

    #[snafu(transparent)]
    Upgrade { source: upgrade::UpgradeError },

    ///
    /// call error (special)
    ///

    #[snafu(display("ic call: {msg}"))]
    Call { msg: String },

    ///
    /// catch-all errors to make the API easier to use
    ///

    #[snafu(transparent)]
    Db { source: ::db::Error },

    #[snafu(transparent)]
    Query { source: ::db_query::Error },

    #[snafu(transparent)]
    State { source: ::core_state::Error },

    #[snafu(transparent)]
    Wasm { source: ::core_wasm::Error },
}

impl From<(RejectionCode, String)> for Error {
    fn from(error: (RejectionCode, String)) -> Self {
        Self::Call { msg: error.1 }
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
    // log!(Log::Info, "call: {method}@{id}");

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
