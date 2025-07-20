pub mod admin;
pub mod canister;
pub mod fixture;
pub mod schema;
pub mod simple;

///
/// Prelude
///

pub(crate) mod prelude {
    pub use crate::schema::{TestDataStore, TestIndexStore};
    pub use mimic::design::{
        base::{types, validator},
        prelude::*,
    };
}
pub use prelude::*;
