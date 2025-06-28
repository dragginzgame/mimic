pub mod admin;
pub mod collections;
pub mod constant;
pub mod db;
pub mod default;
pub mod fixtures;
pub mod index;
pub mod schema;
pub mod sorted;
pub mod validate;

///
/// Prelude
///

pub(crate) mod prelude {
    pub use mimic::core::types::Principal;
    pub use mimic::prelude::*;
    pub use mimic_design::*;
}
pub use prelude::*;

///
/// EntityIdTest
///

#[entity_id(key = "Test")]
pub struct EntityIdTest {}

///
/// TodoUnit
///

#[newtype(item(prim = "Unit", todo), primitive = "Unit")]
pub struct TodoUnit {}

///
/// TodoTarget
///

#[newtype(item(todo, is = "Todo"), primitive = "Nat8")]
pub struct TodoTarget {}

///
/// Todo
///

#[newtype(ty(todo), item(prim = "Nat8"), primitive = "Nat8")]
pub struct Todo {}
