pub mod admin;
pub mod collections;
pub mod constant;
pub mod db;
pub mod default;
pub mod index;
pub mod rarity;
pub mod sorted;
pub mod validate;

///
/// Prelude
///

pub(crate) mod prelude {
    pub use mimic::prelude::*;
    pub use mimic::types::prim::*;
    pub use mimic_base as base;
    pub use mimic_design::*;
}
pub use prelude::*;

///
/// Canister
///

#[canister]
pub struct Canister {}

///
/// Store
///

#[store(ident = "STORE", ty = "Data", canister = "Canister", memory_id = 20)]
pub struct Store {}

///
/// Index
///

#[store(ident = "INDEX", ty = "Index", canister = "Canister", memory_id = 21)]
pub struct Index {}

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

#[newtype(ty(todo), item(is = "Nat8"), primitive = "Nat8")]
pub struct Todo {}
