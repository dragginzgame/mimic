pub mod newtype;
pub mod relation;

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
/// EnumSorted
///

#[enum_(
    variant(name = "A", default),
    variant(name = "B"),
    variant(name = "C"),
    variant(name = "D"),
    traits(add(Sorted))
)]
pub struct EnumSorted {}

///
/// EnumUnspecified
///

#[enum_(
    variant(unspecified, default),
    variant(name = "A"),
    variant(name = "B"),
    variant(name = "C"),
    variant(name = "D")
)]
pub struct EnumUnspecified {}

///
/// DecimalNewtype
///

#[newtype(item(prim = "Decimal"), primitive = "Decimal")]
pub struct DecimalNewtype {}

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

///
/// Negative
/// (just to check on the rust-analyzer error)
///

#[newtype(
    primitive = "Int8",
    item(prim = "Int8"),
    ty(validator(path = "validator::number::Range", args(-1, 3)))
)]
pub struct Negative {}

///
/// Selector
///

#[selector(
    target = "DecimalNewtype",
    variant(name = "Cm50", value = 0.5),
    variant(name = "Metre1", value = 1.0),
    variant(name = "Meter10", value = 10.0)
)]
pub struct Selector {}

///
/// NewtypeValidated
///

#[newtype(
    primitive = "Decimal",
    item(prim = "Decimal"),
    ty(validator(path = "validator::number::Lte", args(5.0)))
)]
pub struct NewtypeValidated {}
