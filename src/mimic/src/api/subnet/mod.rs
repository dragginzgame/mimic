pub mod cascade;
pub mod request;

use crate::core::state::subnet_index::{Error as SubnetIndexError, SubnetIndexManager};
use candid::Principal;
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
    SubnetIndex { source: SubnetIndexError },

    #[snafu(transparent)]
    Schema {
        source: crate::api::core::schema::Error,
    },
}

// user_canister_id
pub fn user_canister_id() -> Result<Principal, Error> {
    let user_canisters = crate::api::core::schema::canisters_by_build(
        crate::orm::schema::node::CanisterBuild::User,
    )?;
    let user_canister = user_canisters.first().ok_or(Error::NoUserCanister)?;

    let user_canister_id = SubnetIndexManager::try_get_canister(&user_canister.def.path())?;

    Ok(user_canister_id)
}
