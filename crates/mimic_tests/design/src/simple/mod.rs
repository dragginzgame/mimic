pub mod newtype;
pub mod sorted;

use crate::prelude::*;

//
// SIMPLE TESTS
// these types just test themselves by existing
//

///
/// List
///

#[list(item(prim = "Text"))]
pub struct List {}

///
/// Map
///

#[map(key(prim = "Text"), value(item(prim = "Nat8")))]
pub struct Map {}

///
/// Set
///

#[set(item(prim = "Text"))]
pub struct Set {}

///
/// Constant
///

#[constant(ty = "Str", value = "Hello")]
pub struct CONSTANT {}

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
