pub mod macros;

///
/// mimic
/// [for external use only]
///
pub use api;
pub use config;
pub use ic;
pub use types;

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

pub mod schema {
    pub use core_schema::{get_schema, Schema};
}

///
/// ::mimic module code
///
use candid::CandidType;
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
    CoreState { source: core_state::Error },

    #[snafu(transparent)]
    CoreWasm { source: core_wasm::Error },
}

///
/// MIMIC PRELUDE
///
/// NOTE: Do not put the candid macros (query, update etc.) directly within this prelude as the endpoints
/// will fail to be registered with the export_candid! macro
///

pub mod prelude {
    pub use crate::{mimic_end, mimic_start, perf};
    pub use ::api::{
        auth::{guard, Guard},
        request::{Request, RequestKind, Response},
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::core_state::{
        AppCommand, AppState, AppStateManager, CanisterState, CanisterStateManager, SubnetIndex,
        SubnetIndexManager, User, UserIndex, UserIndexManager,
    };
    pub use ::db_query;
    pub use ::ic::{caller, format_cycles, id, log, Log};
    pub use ::orm::traits::{EntityFixture, Path};
    pub use ::std::cell::RefCell;
    pub use ::types::Ulid;
}

///
/// ORM PRELUDE
///

pub mod orm {
    pub mod prelude {
        pub use ::candid::CandidType;
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
