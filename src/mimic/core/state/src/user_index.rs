use super::USER_INDEX;
use candid::{CandidType, Principal};
use core_schema::get_schema;
use derive_more::{Deref, DerefMut};
use lib_ic::structures::{memory::VirtualMemory, BTreeMap};
use mimic_derive::Storable;
use orm_schema::node::Role;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::collections::HashSet;
use types::Ulid;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("role '{role}' not found"))]
    RoleNotFound { role: String },

    #[snafu(display("user id '{id}' already exists"))]
    UserExists { id: Principal },

    #[snafu(display("user id '{id}' not found"))]
    UserNotFound { id: Principal },

    #[snafu(display("user already has role '{role}'"))]
    UserHasRole { role: String },

    #[snafu(display("user does not have role '{role}'"))]
    UserDoesNotHaveRole { role: String },

    #[snafu(transparent)]
    Schema { source: core_schema::Error },
}

///
/// UserIndexManager
///

pub struct UserIndexManager {}

impl UserIndexManager {
    // get
    #[must_use]
    pub fn get() -> UserIndex {
        USER_INDEX.with_borrow(|index| index.iter().collect())
    }

    // get_user
    #[must_use]
    pub fn get_user(id: Principal) -> Option<User> {
        USER_INDEX.with_borrow(|index| index.get(&id))
    }

    // try_get_user
    pub fn try_get_user(id: Principal) -> Result<User, Error> {
        let user =
            USER_INDEX.with_borrow(|index| index.get(&id).ok_or(Error::UserNotFound { id }))?;

        Ok(user)
    }

    // register_user
    pub fn register_user(id: Principal, user: User) -> Result<(), Error> {
        USER_INDEX.with_borrow_mut(|index| {
            if index.contains_key(&id) {
                Err(Error::UserExists { id })?;
            }
            index.insert(id, user);

            Ok(())
        })
    }

    // add_role
    pub fn add_role(id: Principal, role: String) -> Result<(), Error> {
        let schema = get_schema().map_err(Error::from)?;

        // check its a valid role
        if schema.get_node::<Role>(&role).is_none() {
            Err(Error::RoleNotFound { role: role.clone() })?;
        }

        // get user
        let mut user = Self::try_get_user(id)?;
        if !user.roles.insert(role.clone()) {
            Err(Error::UserHasRole { role })?;
        }

        // insert new roles
        USER_INDEX.with_borrow_mut(|index| index.insert(id, user));

        Ok(())
    }

    // remove_role
    pub fn remove_role(id: Principal, role: String) -> Result<(), Error> {
        let schema = get_schema().map_err(Error::from)?;

        // check its a valid role
        if schema.get_node::<Role>(&role).is_none() {
            Err(Error::RoleNotFound { role: role.clone() })?;
        }

        // get user
        let mut user = Self::try_get_user(id)?;
        if !user.roles.remove(&role) {
            Err(Error::UserDoesNotHaveRole { role })?;
        }

        // insert new roles
        USER_INDEX.with_borrow_mut(|index| index.insert(id, user));

        Ok(())
    }
}

///
/// UserIndex
/// a map of Principal -> role, authentication and canister assignment data
///

pub type UserIndex = Vec<(Principal, User)>;

///
/// UserIndexStable
///

#[derive(Deref, DerefMut)]
pub struct UserIndexStable(BTreeMap<Principal, User>);

impl UserIndexStable {
    // init
    #[must_use]
    pub fn init(memory: VirtualMemory) -> Self {
        Self(BTreeMap::init(memory))
    }

    // values
    pub fn values(&self) -> impl Iterator<Item = User> + '_ {
        self.0.values()
    }
}

///
/// User
///
/// All the data about a system user at the IC level
/// roles: for Role-based access control
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize, Storable)]
pub struct User {
    pub canister_principal: Principal,
    pub user_id: Ulid,
    pub user_principal: Principal,
    pub roles: HashSet<String>,
}

impl User {
    #[must_use]
    pub fn new(
        canister_principal: Principal,
        user_id: Ulid,
        user_principal: Principal,
        roles: &[String],
    ) -> Self {
        Self {
            canister_principal,
            user_id,
            user_principal,
            roles: roles.iter().cloned().collect(),
        }
    }
}
