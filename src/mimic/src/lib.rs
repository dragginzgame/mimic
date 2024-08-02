///
/// Framework Re-Export
///
/// NOTE: this crate (mimic) is designed to be used by external crates, not internal ones
///
pub use api;
pub use derive;
pub use ic;
pub use orm;
pub use schema;
pub use types;

pub mod core {
    pub use config;
    pub use core_schema as schema;
    pub use state;
    pub use wasm;
}

pub mod db {
    pub use db::*;
    pub use query;
}

pub mod lib {
    pub use lib_case as case;
    pub use lib_cbor as cbor;
    pub use lib_rand as rand;
}

//
// Prelude
//
// NOTE: Do not put the candid macros (query, update etc.) directly within this prelude as the endpoints
// will fail to be registered with the export_candid! macro
//

pub mod prelude {
    pub use crate::{
        api::{
            auth::{guard, Guard},
            mimic_end, mimic_start, perf,
            request::{Request, RequestKind, Response},
        },
        core::state::{
            AppCommand, AppState, AppStateManager, CanisterState, CanisterStateManager,
            SubnetIndex, SubnetIndexManager, User, UserIndex, UserIndexManager,
        },
        db::query,
        ic::{caller, format_cycles, id, log, Log},
        orm::traits::EntityFixture,
    };
    pub use ::candid::Principal;
    pub use ::std::cell::RefCell;
    pub use ::types::Ulid;
}
