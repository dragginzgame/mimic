pub mod config;
pub mod schema;

///
/// Framework Re-Export
///
/// NOTE: this crate (mimic) is designed to be used by external crates, not internal ones
///
pub use api;
pub use ic;
pub use orm;
pub use types;

pub mod core {
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

///
/// PRELUDE
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
        db::query,
        ic::{caller, format_cycles, id, log, Log},
        mimic_end, mimic_start,
        orm::traits::{EntityFixture, Path},
        perf,
    };
    pub use ::candid::{CandidType, Principal};
    pub use ::std::cell::RefCell;
    pub use ::types::Ulid;
}

///
/// MACROS
///

// mimic_start
// macro to be included at the start of each canister lib.rs file
#[macro_export]
macro_rules! mimic_start {
    ($config_path:expr) => {{
        // CONFIG CHECK
        let config_str = include_str!("../../../config.toml");
        ::mimic::config::init_toml(config_str).expect("Failed to load configuration");
        panic!("{}", ::mimic::config::get_config().unwrap());

        include!(concat!("../../../../../generated/actor/", $actor, ".rs"));
    }};
}

// mimic_end
// macro that needs to be included as the last item in the actor lib.rs file
#[macro_export]
macro_rules! mimic_end {
    () => {
        // export_candid
        // has to be at the end
        ::mimic::ic::export_candid!();
    };
}

// perf
#[macro_export]
macro_rules! perf {
    () => {
        ::mimic::api::defer!(::mimic::ic::log!(
            Log::Perf,
            "api call used {} instructions ({})",
            ::mimic::ic::api::performance_counter(1),
            module_path!()
        ));
    };
}
