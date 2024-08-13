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
