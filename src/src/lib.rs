///
/// Framework Re-Export
///
/// NOTE: this crate (mimic) is designed to be used by external crates, not internal ones
///
pub use api;
pub use ic;
pub use orm;
pub use schema;
pub use types;
pub use wasm;

pub mod core {
    pub use core_schema as schema;
    pub use core_state as state;
}

pub mod db {
    pub use db::*;

    pub use query;
}

// lib
pub mod lib {
    pub use lib_case as case;
    pub use lib_rand as rand;
    pub use lib_string as string;
}
