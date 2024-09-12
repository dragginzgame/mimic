pub mod cascade;
pub mod request;

use candid::Principal;
use core_state::SubnetIndexManager;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("no user canister defined in schema"))]
    NoUserCanister,

    #[snafu(transparent)]
    SubnetIndex {
        source: core_state::subnet_index::Error,
    },

    #[snafu(transparent)]
    Schema { source: crate::core::schema::Error },
}

// user_canister_id
pub fn user_canister_id() -> Result<Principal, Error> {
    let user_canisters =
        crate::core::schema::canisters_by_build(::orm_schema::node::CanisterBuild::User)?;
    let user_canister = user_canisters.first().ok_or(Error::NoUserCanister)?;

    let user_canister_id = SubnetIndexManager::try_get_canister(&user_canister.def.path())?;

    Ok(user_canister_id)
}
