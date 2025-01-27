pub mod cascade;
pub mod request;

use crate::{
    api::core::schema::SchemaError,
    core::state::{SubnetIndexError, SubnetIndexManager},
};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// SubnetError
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum SubnetError {
    #[snafu(display("no user canister defined in schema"))]
    NoUserCanister,

    #[snafu(transparent)]
    SubnetIndexError { source: SubnetIndexError },

    #[snafu(transparent)]
    SchemaError { source: SchemaError },
}

// user_canister_id
pub fn user_canister_id() -> Result<Principal, SubnetError> {
    let user_canisters = crate::api::core::schema::canisters_by_build(
        crate::orm::schema::node::CanisterBuild::User,
    )?;
    let user_canister = user_canisters.first().ok_or(SubnetError::NoUserCanister)?;

    let user_canister_id = SubnetIndexManager::try_get_canister(&user_canister.def.path())?;

    Ok(user_canister_id)
}
