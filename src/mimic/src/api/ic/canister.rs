use crate::{core::state::CanisterStateManager, orm::schema::node::Canister};
use candid::Principal;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Schema {
        source: crate::api::core::schema::Error,
    },

    #[snafu(transparent)]
    CanisterState {
        source: crate::core::state::canister_state::Error,
    },
}

// balance
#[must_use]
pub fn balance() -> u128 {
    crate::ic::api::canister_balance128()
}

// caller
#[must_use]
pub fn caller() -> Principal {
    crate::ic::api::caller()
}

// id
#[must_use]
pub fn id() -> Principal {
    crate::ic::api::id()
}

// schema
pub fn schema() -> Result<Canister, Error> {
    let path = path()?;
    let cs = crate::api::core::schema::canister(&path)?;

    Ok(cs)
}

// time
#[must_use]
pub fn time() -> u64 {
    crate::ic::api::time()
}

// version
#[must_use]
pub fn version() -> u64 {
    crate::ic::api::canister_version()
}

///
/// STATE FUNCTIONS
///

// path
pub fn path() -> Result<String, Error> {
    let path = CanisterStateManager::get_path().map_err(Error::from)?;

    Ok(path)
}

// root_id
pub fn root_id() -> Result<Principal, Error> {
    let root_id = CanisterStateManager::get_root_id().map_err(Error::from)?;

    Ok(root_id)
}

// parent_id
#[must_use]
pub fn parent_id() -> Option<Principal> {
    CanisterStateManager::get_parent_id()
}
