#![allow(clippy::wildcard_imports)]
#![allow(clippy::too_many_lines)]
// @todo - rust bug
// https://github.com/intellij-rust/intellij-rust/issues/9853
// remove this from time to time to see the actual unused imports
#![allow(unused_imports)]
#![allow(clippy::too_many_lines)]

pub mod canister;

// prelude
pub mod prelude {
    pub use mimic::orm::prelude::*;
    pub use mimic_base::{
        self as base,
        types::{Principal, Ulid},
    };
}

// init
// schema generation requires a function stub
// to work on OSX
pub const fn init() {}
