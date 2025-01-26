pub mod admin;
pub mod constant;
pub mod default;
pub mod index;
pub mod map;
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
    entity_acl(load = "allow", save = "allow", delete = "allow")
)]
pub struct Store {}

///
/// EntityIdTest
///

#[entity_id(key = "Test")]
pub struct EntityIdTest {}
