pub mod macros;

///
/// mimic
/// [for external use only, keep out of reach of children]
///
pub use api;
pub use build;
pub use db;
pub use ic;
pub use types;

pub mod core {
    pub use core_config as config;
    pub use core_schema as schema;
    pub use core_state as state;
    pub use core_wasm as wasm;
}

pub mod export {
    pub use ctor;
    pub use num_traits;
    pub use remain;
    pub use strum;
}

pub mod orm {
    pub mod prelude {
        pub use ::candid::{CandidType, Principal};
        pub use ::ic::structures::storable::Bound;
        pub use ::lib_case::{Case, Casing};
        pub use ::num_traits::{NumCast, ToPrimitive};
        pub use ::orm::{
            collections::HashSet,
            helper::FixtureList,
            traits::{
                EntityDynamic, EntityFixture, EnumValue, Filterable, Inner, Orderable, Path,
                PrimaryKey, Sanitize, SanitizeManual, Storable, Validate, ValidateManual,
                Visitable,
            },
            Error,
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
            subnet::request::{Request, RequestKind, Response},
            Error, StartupHooks,
        },
        db::Db,
        mimic_end, mimic_start,
        orm::traits::{EntityDynamic, EntityFixture, Path},
        perf,
        types::Ulid,
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::core_state::{
        AppCommand, AppState, AppStateManager, CanisterState, CanisterStateManager, SubnetIndex,
        SubnetIndexManager, User, UserIndex, UserIndexManager,
    };
    pub use ::ic::{caller, format_cycles, id, log, Log};
    pub use ::std::cell::RefCell;
}
