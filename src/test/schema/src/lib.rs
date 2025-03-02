pub mod admin;
pub mod collections;
pub mod constant;
pub mod db;
pub mod default;
pub mod index;
pub mod validate;

use mimic::orm::{base::types, prelude::*};

// init
// schema generation requires a function stub
// to work on OSX
pub const fn init() {}

///
/// Canister
///

#[canister]
pub struct Canister {}

///
/// Store
///

#[store(ident = "STORE", canister = "Canister", memory_id = 20)]
pub struct Store {}

///
/// EntityIdTest
///

#[entity_id(key = "Test")]
pub struct EntityIdTest {}

///
/// TodoUnit
///

#[newtype(item(todo), primitive = "Unit")]
pub struct TodoUnit {}

///
/// TodoTarget
///

#[newtype(item(todo, is = "Todo"), primitive = "U8")]
pub struct TodoTarget {}

///
/// Todo
///

#[newtype(ty(todo), item(is = "types::U8"), primitive = "U8")]
pub struct Todo {}
