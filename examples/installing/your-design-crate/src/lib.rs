pub mod canister;

// prelude
pub mod prelude {
    pub use mimic::orm::prelude::*;
}

// init
// schema generation requires a function stub
// to work on OSX
pub const fn init() {}
