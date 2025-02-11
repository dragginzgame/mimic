pub mod admin;
pub mod constant;
pub mod db;
pub mod default;
pub mod index;
pub mod map;
pub mod validate;

use mimic::orm::{base::types, prelude::*};

// init
// schema generation requires a function stub
// to work on OSX
pub const fn init() {}

///
/// Store
///

#[store(memory_id = 20)]
pub struct Store {}

///
/// EntityIdTest
///

#[entity_id(key = "Test")]
pub struct EntityIdTest {}

///
/// TodoUnit
///

#[newtype(value(item(todo)))]
pub struct TodoUnit {}

///
/// TodoTarget
///

#[newtype(value(item(todo, is = "Todo")))]
pub struct TodoTarget {}

///
/// Todo
///

#[newtype(ty(todo), value(item(is = "types::U8")))]
pub struct Todo {}
