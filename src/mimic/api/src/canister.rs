use crate::Error;
use candid::Principal;
use core_state::CanisterStateManager;
use orm_schema::node::Canister;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// CanisterError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum CanisterError {
    #[snafu(transparent)]
    State { source: core_state::Error },
}

///
/// CANISTER IC FUNCTIONS
///

// balance
#[must_use]
pub fn balance() -> u128 {
    ic::api::canister_balance128()
}

// caller
#[must_use]
pub fn caller() -> Principal {
    ic::api::caller()
}

// id
#[must_use]
pub fn id() -> Principal {
    ic::api::id()
}

// schema
pub fn schema() -> Result<Canister, Error> {
    let path = path()?;
    let cs = crate::schema::canister(&path)?;

    Ok(cs)
}

// time
#[must_use]
pub fn time() -> u64 {
    ic::api::time()
}

// version
#[must_use]
pub fn version() -> u64 {
    ic::api::canister_version()
}

///
/// CANISTER STATE FUNCTIONS
///

// path
pub fn path() -> Result<String, Error> {
    let path = CanisterStateManager::get_path().map_err(CanisterError::from)?;

    Ok(path)
}

// root_id
pub fn root_id() -> Result<Principal, Error> {
    let root_id = CanisterStateManager::get_root_id().map_err(CanisterError::from)?;

    Ok(root_id)
}

// parent_id
#[must_use]
pub fn parent_id() -> Option<Principal> {
    CanisterStateManager::get_parent_id()
}
