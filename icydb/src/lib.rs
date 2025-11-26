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

pub mod base {
    pub use icydb_base::*;
}

pub mod core {
    pub use icydb_core::*;
}

pub mod error {
    pub use icydb_error::*;
}

pub mod macros {
    pub use icydb_macros::*;
}

//
// Actor Prelude
//

pub mod prelude {
    pub use icydb_core::{Value, db::primitives::FilterDsl};
    pub use icydb_macros::*;
}

//
// Design Prelude
// For schema/design code (macros, traits, base helpers).
//

pub mod design {
    pub mod prelude {
        pub use icydb_core::design::prelude::*;
    }
}
