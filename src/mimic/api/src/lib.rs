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
pub use ic::api::call::call;

use candid::CandidType;
use ic::api::call::RejectionCode;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

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
    Subnet { source: subnet::SubnetError },

    #[snafu(transparent)]
    Upgrade { source: upgrade::UpgradeError },

    ///
    /// call error (special)
    ///

    #[snafu(display("ic call: {msg}"))]
    Call { msg: String },
}

impl From<(RejectionCode, String)> for Error {
    fn from(error: (RejectionCode, String)) -> Self {
        Self::Call { msg: error.1 }
    }
}
