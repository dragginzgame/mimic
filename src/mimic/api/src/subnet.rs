use crate::Error;
use candid::Principal;
use core_state::SubnetIndexManager;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// SubnetError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum SubnetError {
    #[snafu(display("no user canister defined in schema"))]
    NoUserCanister,

    #[snafu(transparent)]
    CoreState { source: core_state::Error },
}

// user_canister_id
pub fn user_canister_id() -> Result<Principal, Error> {
    let user_canisters =
        crate::schema::canisters_by_build(::orm_schema::node::CanisterBuild::User)?;
    let user_canister = user_canisters.first().ok_or(SubnetError::NoUserCanister)?;

    let user_canister_id = SubnetIndexManager::try_get_canister(&user_canister.def.path())
        .map_err(SubnetError::from)?;

    Ok(user_canister_id)
}
