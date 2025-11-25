// icydb/src/lib.rs
//! IcyDB — Data Models, Queries, and IC Integration for Canisters
//!
//! This is the public meta-crate. Downstream users depend on **icydb** only.
//!
//! It re-exports the stable public API from:
//!   - `icydb-core`     (runtime data model, filters, queries, values…)
//!   - `icydb-macros`   (derive macros)
//!   - `icydb-ic`       (IC/CDK glue, entrypoints, canister helpers)
//!
//! Everything else (`icydb-schema`, `icydb-build`) is internal.

//
// ---- Re-export derive macros ------------------------------------------------
//
pub mod macros {
    pub use icydb_macros::*;
}

//
// ---- Re-export base ---
//
pub mod base {
    pub use icydb_base::*;
}

//
// ---- Re-export runtime model / query API -----------------------------------
//
pub mod core {
    pub use icydb_core::*;
}

//
// ---- Re-export IC adapter layer (optional for non-IC users) -----------------
//
pub mod ic {
    pub use icydb_ic::*;
}

//
// Third party re-exports
//

pub mod export {
    pub use canic;
    pub use ctor;
    pub use derive_more;
    pub use num_traits;
    pub use remain;
}

//
// ---- Crate-level Prelude ----------------------------------------------------
//
// Users can import `use icydb::prelude::*;` for the common API surface.
//

pub mod prelude {
    // Derive macros
    pub use icydb_macros::*;

    // Core runtime API
    pub use icydb_core::{core::Value, db::primitives::FilterDsl};

    // IC integration (only when feature enabled)
    pub use icydb_ic::{
        icydb_start,
        // other public canister helpers
    };
}
