pub mod auth;
pub mod core;
pub mod guard;
pub mod ic;
pub mod subnet;

use crate::{
    api::{
        ic::{
            call::CallError as IcCallError, canister::CanisterError as IcCanisterError,
            create::CreateError as IcCreateError, upgrade::UpgradeError as IcUpgradeError,
        },
        subnet::{
            cascade::CascadeError as SubnetCascadeError,
            request::RequestError as SubnetRequestError,
        },
    },
    core::{
        config::ConfigError as CoreConfigError,
        schema::SchemaError as CoreSchemaError,
        state::{
            AppStateError as CoreAppStateError, CanisterStateError as CoreCanisterStateError,
            ChildIndexError as CoreChildIndexError, SubnetIndexError as CoreSubnetIndexError,
            UserIndexError as CoreUserIndexError,
        },
        wasm::WasmError as CoreWasmError,
    },
    query::{
        DeleteError as QueryDeleteError, LoadError as QueryLoadError, QueryError,
        SaveError as QuerySaveError,
    },
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{
    error::Error as StdError,
    fmt::{self, Display},
    future::Future,
};

///
/// StartupHooks
///

pub trait StartupHooks {
    // startup
    // on every startup regardless of installation mode
    fn startup() -> Result<(), ApiError> {
        Ok(())
    }

    // init
    // custom code called after mimic init()
    fn init() -> Result<(), ApiError> {
        Ok(())
    }

    // init_async
    // custom code called after mimic init_async()
    #[must_use]
    fn init_async() -> impl Future<Output = Result<(), ApiError>> + Send {
        async { Ok(()) }
    }

    // pre_upgrade
    // called after pre_upgrade
    fn pre_upgrade() -> Result<(), ApiError> {
        Ok(())
    }

    // post_upgrade
    // called after post_upgrade
    fn post_upgrade() -> Result<(), ApiError> {
        Ok(())
    }
}

///
/// ERROR
/// consolidates all the different crate errors into one place
///

// misc
pub const ERROR_INIT: u8 = 10;

// api
pub const ERROR_API_AUTH: u8 = 101;
pub const ERROR_API_CORE_SCHEMA: u8 = 110;
pub const ERROR_API_IC_CALL: u8 = 120;
pub const ERROR_API_IC_CANISTER: u8 = 121;
pub const ERROR_API_IC_CREATE: u8 = 122;
pub const ERROR_API_IC_MGMT: u8 = 123;
pub const ERROR_API_IC_UPGRADE: u8 = 124;
pub const ERROR_API_SUBNET_CASCADE: u8 = 130;
pub const ERROR_API_SUBNET_REQUEST: u8 = 131;

// core
pub const ERROR_CORE_CONFIG: u8 = 140;
pub const ERROR_CORE_SCHEMA: u8 = 141;
pub const ERROR_CORE_WASM: u8 = 142;
pub const ERROR_CORE_APP_STATE: u8 = 143;
pub const ERROR_CORE_CANISTER_STATE: u8 = 144;
pub const ERROR_CORE_CHILD_INDEX: u8 = 145;
pub const ERROR_CORE_SUBNET_INDEX: u8 = 146;
pub const ERROR_CORE_USER_INDEX: u8 = 147;

// orm
pub const ERROR_ORM: u8 = 150;

// db
pub const ERROR_DB: u8 = 160;

// query
pub const ERROR_DB_QUERY: u8 = 170;
pub const ERROR_DB_QUERY_LOAD: u8 = 171;
pub const ERROR_DB_QUERY_SAVE: u8 = 172;
pub const ERROR_DB_QUERY_DELETE: u8 = 173;

///
/// ApiError
///

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct ApiError(u8, String);

impl ApiError {
    #[must_use]
    pub const fn init(text: String) -> Self {
        Self(ERROR_INIT, text)
    }

    #[must_use]
    pub const fn new(code: u8, text: String) -> Self {
        Self(code, text)
    }
}

impl StdError for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

//
// api
//

impl From<auth::AuthError> for ApiError {
    fn from(error: auth::AuthError) -> Self {
        Self(ERROR_API_AUTH, error.to_string())
    }
}

//
// api ic
//

impl From<IcCallError> for ApiError {
    fn from(error: IcCallError) -> Self {
        Self(ERROR_API_IC_CALL, error.to_string())
    }
}

impl From<IcCanisterError> for ApiError {
    fn from(error: IcCanisterError) -> Self {
        Self(ERROR_API_IC_CANISTER, error.to_string())
    }
}

impl From<IcCreateError> for ApiError {
    fn from(error: IcCreateError) -> Self {
        Self(ERROR_API_IC_CREATE, error.to_string())
    }
}

impl From<IcUpgradeError> for ApiError {
    fn from(error: IcUpgradeError) -> Self {
        Self(ERROR_API_IC_UPGRADE, error.to_string())
    }
}

//
// api subnet
//

impl From<SubnetCascadeError> for ApiError {
    fn from(error: SubnetCascadeError) -> Self {
        Self(ERROR_API_SUBNET_CASCADE, error.to_string())
    }
}

impl From<SubnetRequestError> for ApiError {
    fn from(error: SubnetRequestError) -> Self {
        Self(ERROR_API_SUBNET_REQUEST, error.to_string())
    }
}

//
// core
//

impl From<CoreConfigError> for ApiError {
    fn from(error: CoreConfigError) -> Self {
        Self(ERROR_CORE_CONFIG, error.to_string())
    }
}

impl From<CoreSchemaError> for ApiError {
    fn from(error: CoreSchemaError) -> Self {
        Self(ERROR_CORE_SCHEMA, error.to_string())
    }
}

impl From<CoreAppStateError> for ApiError {
    fn from(error: CoreAppStateError) -> Self {
        Self(ERROR_CORE_APP_STATE, error.to_string())
    }
}

impl From<CoreCanisterStateError> for ApiError {
    fn from(error: CoreCanisterStateError) -> Self {
        Self(ERROR_CORE_CANISTER_STATE, error.to_string())
    }
}

impl From<CoreChildIndexError> for ApiError {
    fn from(error: CoreChildIndexError) -> Self {
        Self(ERROR_CORE_CHILD_INDEX, error.to_string())
    }
}

impl From<CoreSubnetIndexError> for ApiError {
    fn from(error: CoreSubnetIndexError) -> Self {
        Self(ERROR_CORE_SUBNET_INDEX, error.to_string())
    }
}

impl From<CoreUserIndexError> for ApiError {
    fn from(error: CoreUserIndexError) -> Self {
        Self(ERROR_CORE_USER_INDEX, error.to_string())
    }
}

impl From<CoreWasmError> for ApiError {
    fn from(error: CoreWasmError) -> Self {
        Self(ERROR_CORE_WASM, error.to_string())
    }
}

//
// orm
//

impl From<crate::orm::OrmError> for ApiError {
    fn from(error: crate::orm::OrmError) -> Self {
        Self(ERROR_ORM, error.to_string())
    }
}

//
// db
//

impl From<crate::db::DbError> for ApiError {
    fn from(error: crate::db::DbError) -> Self {
        Self(ERROR_DB, error.to_string())
    }
}

//
// query
//

impl From<QueryError> for ApiError {
    fn from(error: crate::query::QueryError) -> Self {
        Self(ERROR_DB_QUERY, error.to_string())
    }
}

impl From<QueryLoadError> for ApiError {
    fn from(error: QueryLoadError) -> Self {
        Self(ERROR_DB_QUERY_LOAD, error.to_string())
    }
}

impl From<QuerySaveError> for ApiError {
    fn from(error: QuerySaveError) -> Self {
        Self(ERROR_DB_QUERY_SAVE, error.to_string())
    }
}

impl From<QueryDeleteError> for ApiError {
    fn from(error: QueryDeleteError) -> Self {
        Self(ERROR_DB_QUERY_DELETE, error.to_string())
    }
}
