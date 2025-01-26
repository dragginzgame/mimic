use super::USER_INDEX;
use crate::{
    core::schema::{get_schema, SchemaError},
    ic::structures::{
        memory::VirtualMemory,
        serialize::{from_binary, to_binary},
        storable::Bound,
        BTreeMap, Storable,
    },
    orm::{base::types::Ulid, schema::node::Role},
};
use candid::{CandidType, Principal};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::{borrow::Cow, collections::HashSet};

///
/// UserIndexError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum UserIndexError {
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
    SchemaError { source: SchemaError },
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
    pub fn try_get_user(id: Principal) -> Result<User, UserIndexError> {
        let user = USER_INDEX
            .with_borrow(|index| index.get(&id).ok_or(UserIndexError::UserNotFound { id }))?;

        Ok(user)
    }

    // register_user
    pub fn register_user(id: Principal, user: User) -> Result<(), UserIndexError> {
        USER_INDEX.with_borrow_mut(|index| {
            if index.contains_key(&id) {
                Err(UserIndexError::UserExists { id })?;
            }
            index.insert(id, user);

            Ok(())
        })
    }

    // add_role
    pub fn add_role(id: Principal, role: String) -> Result<(), UserIndexError> {
        let schema = get_schema()?;

        // check its a valid role
        if schema.get_node::<Role>(&role).is_none() {
            Err(UserIndexError::RoleNotFound { role: role.clone() })?;
        }

        // get user
        let mut user = Self::try_get_user(id)?;
        if !user.roles.insert(role.clone()) {
            Err(UserIndexError::UserHasRole { role })?;
        }

        // insert new roles
        USER_INDEX.with_borrow_mut(|index| index.insert(id, user));

        Ok(())
    }

    // remove_role
    pub fn remove_role(id: Principal, role: String) -> Result<(), UserIndexError> {
        let schema = get_schema()?;

        // check its a valid role
        if schema.get_node::<Role>(&role).is_none() {
            Err(UserIndexError::RoleNotFound { role: role.clone() })?;
        }

        // get user
        let mut user = Self::try_get_user(id)?;
        if !user.roles.remove(&role) {
            Err(UserIndexError::UserDoesNotHaveRole { role })?;
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

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
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

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(to_binary(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        from_binary(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
