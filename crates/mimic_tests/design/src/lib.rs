#![allow(non_camel_case_types)]

pub mod admin;
pub mod canister;
pub mod fixture;
pub mod schema;
pub mod simple;
pub mod view;

///
/// Prelude
///

pub(crate) mod prelude {
    pub use mimic::design::{
        base::{types, validator},
        prelude::*,
    };
}
pub use prelude::*;
