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
/// EntityIdTest
///

#[entity_id(key = "Test")]
pub struct EntityIdTest {}

///
/// TodoTest
///

#[newtype(value(item(todo)))]
pub struct TodoTest {}

///
/// TodoTestTarget
///

#[newtype(value(item(todo, is = "Todo")))]
pub struct TodoTestTarget {}

///
/// Todo
///

#[newtype(ty(todo), value(item(is = "types::U8")))]
pub struct Todo {}
