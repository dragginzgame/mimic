use crate::{
    api::core::schema::SchemaError,
    core::state::{CanisterStateError, CanisterStateManager},
    orm::schema::node::Canister,
    DynError,
};
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// CanisterError
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum CanisterError {
    #[snafu(transparent)]
    SchemaError { source: SchemaError },

    #[snafu(transparent)]
    CanisterStateError { source: CanisterStateError },
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
pub fn schema() -> Result<Canister, DynError> {
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
pub fn path() -> Result<String, DynError> {
    let path = CanisterStateManager::get_path()?;

    Ok(path)
}

// root_id
pub fn root_id() -> Result<Principal, DynError> {
    let root_id = CanisterStateManager::get_root_id()?;

    Ok(root_id)
}

// parent_id
#[must_use]
pub fn parent_id() -> Option<Principal> {
    CanisterStateManager::get_parent_id()
}
