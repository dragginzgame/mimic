///
/// mimic
/// [for external use only, keep out of reach of children]
///
pub mod config;
pub mod db;
pub mod macros;
pub mod orm;
pub mod query;
pub mod schema;
pub mod types;
pub mod utils;

pub mod ic {
    pub use icu::ic::*;
}

pub mod export {
    pub use ctor;
    pub use derive_more;
    pub use num_traits;
    pub use remain;
}

extern crate self as mimic;

///
/// MIMIC PRELUDE
///

pub mod prelude {
    pub use crate::{
        db::Store,
        mimic_end, mimic_memory_manager, mimic_start,
        orm::{
            base::{
                self,
                types::{Relation, RelationSet, Ulid},
            },
            traits::{
                EntityDyn, EntityFixture, Inner as _, NumFromPrimitive, NumToPrimitive, Path,
            },
        },
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::icu::{
        Log,
        ic::{
            api::{canister_self, msg_caller},
            call::Call,
            init, query, update,
        },
        log, perf,
    };
    pub use ::std::cell::RefCell;
}

use candid::CandidType;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum Error {
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    DbError(#[from] db::DbError),

    #[error(transparent)]
    OrmError(#[from] orm::OrmError),

    #[error(transparent)]
    QueryError(#[from] query::QueryError),

    #[error(transparent)]
    SchemaError(#[from] schema::SchemaError),
}
