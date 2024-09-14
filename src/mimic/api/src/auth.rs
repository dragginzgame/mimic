use crate::ic::call::call;
use candid::Principal;
use core_state::{ChildIndexManager, SubnetIndexManager};
use lib_ic::{api::is_controller, caller};
use orm_schema::node::AccessPolicy;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("one or more rules must be defined"))]
    NoRulesDefined,

    #[snafu(display("there has to be a user canister defined in the schema"))]
    NoUserCanister,

    #[snafu(display("this action is not allowed due to configuration settings"))]
    NotAllowed,

    #[snafu(display("principal '{id}' does not match canister path '{path}'"))]
    NotCanisterPath { id: Principal, path: String },

    #[snafu(display("principal '{id}' is not a child of this canister'"))]
    NotChild { id: Principal },

    #[snafu(display("principal '{id}' is not a controller of this canister'"))]
    NotController { id: Principal },

    #[snafu(display("principal '{id}' is not the parent of this canister'"))]
    NotParent { id: Principal },

    #[snafu(display("principal '{id}' does not have the permission '{permission}'"))]
    NotPermitted { id: Principal, permission: String },

    #[snafu(display("principal '{id}' is not root"))]
    NotRoot { id: Principal },

    #[snafu(display("principal '{id}' is not from this subnet"))]
    NotSubnet { id: Principal },

    #[snafu(display("principal '{id}' is not the current canister"))]
    NotThis { id: Principal },

    #[snafu(display("role '{role}' not found"))]
    RoleNotFound { role: String },

    #[snafu(transparent)]
    Call { source: crate::ic::call::Error },

    #[snafu(transparent)]
    Canister { source: crate::ic::canister::Error },

    #[snafu(transparent)]
    Create { source: crate::ic::create::Error },

    #[snafu(transparent)]
    Subnet { source: crate::subnet::Error },
}

///
/// Guard
///

#[remain::sorted]
pub enum Guard {
    CanisterPath(String),
    Child,
    Controller,
    Parent,
    Permission(String),
    Policy(AccessPolicy),
    Root,
    Subnet,
    This,
}

impl Guard {
    pub async fn result(self, id: Principal) -> Result<(), Error> {
        match self {
            Self::CanisterPath(path) => guard_canister_type(id, &path),
            Self::Child => guard_child(id),
            Self::Controller => guard_controller(id),
            Self::Parent => guard_parent(id),
            Self::Permission(path) => guard_permission(id, &path).await,
            Self::Policy(req) => guard_policy(id, &req).await,
            Self::Root => guard_root(id),
            Self::Subnet => guard_subnet(id).await,
            Self::This => guard_this(id),
        }
    }
}

// guard
pub async fn guard(rules: Vec<Guard>) -> Result<(), Error> {
    // only works for caller now
    let caller = caller();

    // in case rules are accidentally blank / commented out
    if rules.is_empty() {
        Err(Error::NoRulesDefined)?;
    }

    // check rules
    let mut last_error = None;
    for rule in rules {
        match rule.result(caller).await {
            Ok(()) => return Ok(()),
            Err(e) => last_error = Some(e),
        }
    }

    last_error.map_or(Ok(()), Err)
}

///
/// GUARD MACROS
///

// guard_canister_type
// check caller against the id of a specific canister path
fn guard_canister_type(id: Principal, canister_path: &str) -> Result<(), Error> {
    SubnetIndexManager::try_get_canister(canister_path).map_err(|_| Error::NotCanisterPath {
        id,
        path: canister_path.to_string(),
    })?;

    Ok(())
}

// guard_child
fn guard_child(id: Principal) -> Result<(), Error> {
    match ChildIndexManager::try_get_canister(id) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::NotChild { id })?,
    }
}

// guard_controller
fn guard_controller(id: Principal) -> Result<(), Error> {
    if is_controller(&id) {
        Ok(())
    } else {
        Err(Error::NotController { id })?
    }
}

// guard_root
fn guard_root(id: Principal) -> Result<(), Error> {
    let root_id = crate::ic::canister::root_id()?;

    if id == root_id {
        Ok(())
    } else {
        Err(Error::NotRoot { id })?
    }
}

// guard_parent
fn guard_parent(id: Principal) -> Result<(), Error> {
    match crate::ic::canister::parent_id() {
        Some(parent_id) if parent_id == id => Ok(()),
        _ => Err(Error::NotParent { id })?,
    }
}

// guard_permission
// will find the user canister from the schema
pub async fn guard_permission(id: Principal, permission: &str) -> Result<(), Error> {
    let user_canister_id = crate::subnet::user_canister_id()?;

    call::<_, (Result<(), crate::ic::call::Error>,)>(
        user_canister_id,
        "guard_permission",
        (id, permission),
    )
    .await?
    .0?;

    Ok(())
}

// guard_policy
// only from non-PlayerHub canisters
async fn guard_policy(id: Principal, policy: &AccessPolicy) -> Result<(), Error> {
    match policy {
        AccessPolicy::Allow => Ok(()),
        AccessPolicy::Deny => Err(Error::NotAllowed)?,
        AccessPolicy::Permission(permission) => guard_permission(id, permission).await,
    }
}

// guard_subnet
#[expect(clippy::unused_async)]
pub async fn guard_subnet(_id: Principal) -> Result<(), Error> {
    // @todo - we need gabriel code here

    Ok(())
}

// guard_this
fn guard_this(id: Principal) -> Result<(), Error> {
    if id == lib_ic::api::id() {
        Ok(())
    } else {
        Err(Error::NotThis { id })?
    }
}
