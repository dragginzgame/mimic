pub mod admin;
pub mod constant;
pub mod db;
pub mod default;
pub mod index;
pub mod map;
pub mod validate;

use mimic::orm::prelude::*;

// init
// schema generation requires a function stub
// to work on OSX
pub const fn init() {}

///
/// EntityIdTest
///

#[entity_id(key = "Test")]
pub struct EntityIdTest {}
