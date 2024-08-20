pub mod macros;

///
/// mimic
/// [for external use only]
///
pub use api;
pub use build;
pub use config;
pub use ic;
pub use types;

pub mod core {
    pub use core_schema as schema;
    pub use core_state as state;
    pub use core_wasm as wasm;
}

pub mod db {
    pub use db::*;
    pub use db_query as query;
}

pub mod lib {
    pub use lib_case as case;
    pub use lib_cbor as cbor;
    pub use lib_rand as rand;
    pub use lib_time as time;
}

pub mod orm {
    pub mod prelude {
        pub use ::candid::{CandidType, Principal};
        pub use ::ic::structures::storable::Bound;
        pub use ::lib_case::{Case, Casing};
        pub use ::num_traits::{NumCast, ToPrimitive};
        pub use ::orm::{
            collections::HashSet,
            traits::{
                EntityDynamic, EntityFixture, EnumHash, Filterable, Inner, Orderable, Path,
                PrimaryKey, Sanitize, Storable, Validate, Visitable,
            },
        };
        pub use ::orm_macros::*;
        pub use ::serde::{Deserialize, Serialize};
        pub use ::snafu::Snafu;
        pub use ::std::{cmp::Ordering, fmt::Display};
        pub use ::types::ErrorVec;
    }

    pub use orm::*;
    pub use orm_macros as macros;
    pub use orm_schema as schema;
}

pub mod schema {
    pub use core_schema::{get_schema, Schema};
}

pub mod export {
    pub use ctor;
    pub use derive_more;
    pub use num_traits;
    pub use remain;
    pub use strum;
}

///
/// MIMIC PRELUDE
///
/// NOTE: Do not put the candid macros (query, update etc.) directly within this prelude as the endpoints
/// will fail to be registered with the export_candid! macro
///

pub mod prelude {
    pub use crate::{
        api::{
            auth::{guard, Guard},
            request::{Request, RequestKind, Response},
        },
        core::state::{
            AppCommand, AppState, AppStateManager, CanisterState, CanisterStateManager,
            SubnetIndex, SubnetIndexManager, User, UserIndex, UserIndexManager,
        },
        db::query as db_query,
        ic::{caller, format_cycles, id, log, Log},
        mimic_end, mimic_start,
        orm::traits::{EntityFixture, Path},
        perf,
        types::Ulid,
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::std::cell::RefCell;
}

///
/// ::mimic module code
///
use candid::CandidType;
use ic::api::call::RejectionCode;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// ERROR
///
/// consolidates all the different crate errors into one place
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Api { source: api::Error },

    #[snafu(transparent)]
    Config { source: config::Error },

    #[snafu(transparent)]
    Db { source: db::Error },

    #[snafu(transparent)]
    Query { source: db_query::Error },

    #[snafu(transparent)]
    CoreSchema { source: core_schema::Error },

    #[snafu(transparent)]
    CoreState { source: core_state::Error },

    #[snafu(transparent)]
    CoreWasm { source: core_wasm::Error },

    ///
    /// call error (special)
    ///

    #[snafu(display("ic call: {msg}"))]
    Call { msg: String },
}

impl From<(RejectionCode, String)> for Error {
    fn from(error: (RejectionCode, String)) -> Self {
        Self::Call { msg: error.1 }
    }
}
