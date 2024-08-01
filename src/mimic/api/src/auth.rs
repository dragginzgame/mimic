use crate::Error;
use candid::{CandidType, Principal};
use schema::node::AccessPolicy;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use state::{ChildIndexManager, SubnetIndexManager};

///
/// AuthError
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum AuthError {
    #[snafu(display("one or more rules must be defined"))]
    NoRulesDefined,

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
    let caller = ::ic::caller();

    // in case rules are accidentally blank / commented out
    if rules.is_empty() {
        Err(AuthError::NoRulesDefined)?;
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
    SubnetIndexManager::try_get_canister(canister_path).map_err(|_| {
        AuthError::NotCanisterPath {
            id,
            path: canister_path.to_string(),
        }
    })?;

    Ok(())
}

// guard_child
fn guard_child(id: Principal) -> Result<(), Error> {
    match ChildIndexManager::try_get_canister(id) {
        Ok(_) => Ok(()),
        Err(_) => Err(AuthError::NotChild { id })?,
    }
}

// guard_controller
fn guard_controller(id: Principal) -> Result<(), Error> {
    if ::ic::api::is_controller(&id) {
        Ok(())
    } else {
        Err(AuthError::NotController { id })?
    }
}

// guard_root
fn guard_root(id: Principal) -> Result<(), Error> {
    let root_id = crate::canister::root_id()?;

    if id == root_id {
        Ok(())
    } else {
        Err(AuthError::NotRoot { id })?
    }
}

// guard_parent
fn guard_parent(id: Principal) -> Result<(), Error> {
    match crate::canister::parent_id() {
        Some(parent_id) if parent_id == id => Ok(()),
        _ => Err(AuthError::NotParent { id })?,
    }
}

// guard_permission
pub async fn guard_permission(id: Principal, permission: &str) -> Result<(), Error> {
    let user_canister_id = SubnetIndexManager::try_get_canister("::design::canister::user::User")?;

    crate::call::<_, (Result<(), Error>,)>(
        user_canister_id,
        "guard_permission",
        ((id, permission),),
    )
    .await
    .map_err(Error::from)?
    .0?;

    Ok(())
}

// guard_policy
// only from non-PlayerHub canisters
async fn guard_policy(id: Principal, policy: &AccessPolicy) -> Result<(), Error> {
    match policy {
        AccessPolicy::Allow => Ok(()),
        AccessPolicy::Deny => Err(AuthError::NotAllowed)?,
        AccessPolicy::Permission(permission) => guard_permission(id, permission).await,
    }
}

// guard_subnet
pub async fn guard_subnet(_id: Principal) -> Result<(), Error> {
    // @todo - we need gabriel code here

    Ok(())
}

// guard_this
fn guard_this(id: Principal) -> Result<(), Error> {
    if id == ::ic::api::id() {
        Ok(())
    } else {
        Err(AuthError::NotThis { id })?
    }
}
