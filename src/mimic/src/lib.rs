pub mod macros;

///
/// mimic
/// [for external use only]
///
pub use api;
pub use build;
pub use db;
pub use types;

// re-export libraries
pub mod lib {
    pub use lib_case as case;
    pub use lib_ic as ic;
    pub use lib_rand as rand;
    pub use lib_time as time;
}

pub mod orm {
    pub mod prelude {
        pub use ::candid::{CandidType, Principal};
        pub use ::lib_case::{Case, Casing};
        pub use ::lib_ic::structures::storable::Bound;
        pub use ::num_traits::{NumCast, ToPrimitive};
        pub use ::orm::{
            collections::HashSet,
            traits::{
                EntityDynamic, EntityFixture, EnumHash, EnumValue, Filterable, Inner, Orderable,
                Path, PrimaryKey, Sanitize, Storable, Validate, Visitable,
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

pub mod export {
    pub use ctor;
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
            core::state::{
                AppCommand, AppState, AppStateManager, CanisterState, CanisterStateManager,
                SubnetIndex, SubnetIndexManager, User, UserIndex, UserIndexManager,
            },
            subnet::request::{Request, RequestKind, Response},
        },
        db::query as db_query,
        mimic_end, mimic_start,
        orm::traits::{EntityFixture, Path},
        perf,
        types::Ulid,
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::lib_ic::{caller, format_cycles, id, log, Log};
    pub use ::std::cell::RefCell;
}
