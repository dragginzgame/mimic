use crate::{
    api::{
        ic::{
            call::{call, CallError},
            canister::CanisterError,
            create::CreateError,
        },
        subnet::SubnetError,
    },
    core::state::{ChildIndexManager, SubnetIndexManager},
    ic::{api::is_controller, caller},
    orm::schema::node::AccessPolicy,
    DynError, Error,
};
use candid::Principal;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// AuthError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum AuthError {
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
    Error { source: Error },

    #[snafu(transparent)]
    CallError { source: CallError },

    #[snafu(transparent)]
    CanisterError { source: CanisterError },

    #[snafu(transparent)]
    CreateError { source: CreateError },

    #[snafu(transparent)]
    SubnetError { source: SubnetError },
}

///
/// Auth
///

#[remain::sorted]
pub enum Auth {
    CanisterType(String),
    Child,
    Controller,
    Parent,
    Permission(String),
    Policy(AccessPolicy),
    Root,
    SameCanister,
    SameSubnet,
}

impl Auth {
    pub async fn result(self, id: Principal) -> Result<(), DynError> {
        match self {
            Self::CanisterType(path) => rule_canister_type(id, &path),
            Self::Child => rule_child(id),
            Self::Controller => rule_controller(id),
            Self::Parent => rule_parent(id),
            Self::Permission(path) => rule_permission(id, &path).await,
            Self::Policy(req) => rule_policy(id, &req).await,
            Self::Root => rule_root(id),
            Self::SameSubnet => rule_same_subnet(id).await,
            Self::SameCanister => rule_same_canister(id),
        }
    }
}

// allow_any
pub async fn allow_any(rules: Vec<Auth>) -> Result<(), DynError> {
    // only works for caller now
    let caller = caller();

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
/// RULE MACROS
///

// rule_canister_type
// check caller against the id of a specific canister path
fn rule_canister_type(id: Principal, canister_path: &str) -> Result<(), DynError> {
    SubnetIndexManager::try_get_canister(canister_path).map_err(|_| {
        AuthError::NotCanisterPath {
            id,
            path: canister_path.to_string(),
        }
    })?;

    Ok(())
}

// rule_child
fn rule_child(id: Principal) -> Result<(), DynError> {
    match ChildIndexManager::try_get_canister(id) {
        Ok(_) => Ok(()),
        Err(_) => Err(AuthError::NotChild { id })?,
    }
}

// rule_controller
fn rule_controller(id: Principal) -> Result<(), DynError> {
    if is_controller(&id) {
        Ok(())
    } else {
        Err(AuthError::NotController { id })?
    }
}

// rule_root
fn rule_root(id: Principal) -> Result<(), DynError> {
    let root_id = crate::api::ic::canister::root_id()?;

    if id == root_id {
        Ok(())
    } else {
        Err(AuthError::NotRoot { id })?
    }
}

// rule_parent
fn rule_parent(id: Principal) -> Result<(), DynError> {
    match crate::api::ic::canister::parent_id() {
        Some(parent_id) if parent_id == id => Ok(()),
        _ => Err(AuthError::NotParent { id })?,
    }
}

// rule_permission
// will find the user canister from the schema
pub async fn rule_permission(id: Principal, permission: &str) -> Result<(), DynError> {
    let user_canister_id = crate::api::subnet::user_canister_id()?;

    call::<_, (Result<(), ::mimic::Error>,)>(
        user_canister_id,
        "guard_permission",
        (id, permission),
    )
    .await?
    .0
    .map_err(|_| AuthError::NotPermitted {
        id,
        permission: permission.to_string(),
    })?;

    Ok(())
}

// rule_policy
// only from non-PlayerHub canisters
async fn rule_policy(id: Principal, policy: &AccessPolicy) -> Result<(), DynError> {
    match policy {
        AccessPolicy::Allow => Ok(()),
        AccessPolicy::Deny => Err(AuthError::NotAllowed)?,
        AccessPolicy::Permission(permission) => rule_permission(id, permission).await,
    }
}

// rule_same_subnet
#[expect(clippy::unused_async)]
pub async fn rule_same_subnet(_id: Principal) -> Result<(), DynError> {
    // @todo - we need gabriel code here

    Ok(())
}

// rule_same_canister
fn rule_same_canister(id: Principal) -> Result<(), DynError> {
    if id == crate::ic::api::id() {
        Ok(())
    } else {
        Err(AuthError::NotThis { id })?
    }
}
