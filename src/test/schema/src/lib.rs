pub mod admin;
pub mod default;
pub mod map;
pub mod sanitize;
pub mod store;
pub mod validate;

use mimic::orm::prelude::*;

// init
// schema generation requires a function stub
// to work on OSX
pub const fn init() {}

///
/// Canister
///

#[canister(build = "test", initial_cycles = "5T", min_cycles = "5T")]
pub struct Test {}

///
/// Store
///

#[store(
    canister = "Test",
    memory_id = 20,
    crud(load = "allow", save = "allow", delete = "allow")
)]
pub struct Store {}
