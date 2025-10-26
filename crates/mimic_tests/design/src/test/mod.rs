pub mod collection;
pub mod entity;
pub mod newtype;
pub mod relation;
pub mod sanitize;
pub mod validate;

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
/// EntityIdTest
///

#[entity_id(key = "Test")]
pub struct EntityIdTest {}

///
/// EnumSorted
///

#[enum_(
    variant(ident = "A", default),
    variant(ident = "B"),
    variant(ident = "C"),
    variant(ident = "D"),
    traits(add(Sorted))
)]
pub struct EnumSorted {}

///
/// EnumUnspecified
///

#[enum_(
    variant(unspecified, default),
    variant(ident = "A"),
    variant(ident = "B"),
    variant(ident = "C"),
    variant(ident = "D")
)]
pub struct EnumUnspecified {}

///
/// Negative
/// (just to check on the rust-analyzer error)
///

#[newtype(
    primitive = "Int8",
    item(prim = "Int8"),
    ty(validator(path = "validator::num::Range", args(-1, 3)))
)]
pub struct Negative {}

///
/// Selector
///

#[selector(
    target = "SelectorNewtype",
    variant(name = "50 cm", value = 0.5),
    variant(name = "1m", value = 1.0),
    variant(name = "10m", value = 10.0)
)]
pub struct Selector {}

///
/// SelectorRecord
///

#[record(fields(field(
    ident = "interval",
    value(item(is = "SelectorNewtype", selector = "Selector"))
)))]
pub struct SelectorRecord {}

///
/// SelectorNewtype
///

#[newtype(item(prim = "Decimal"), primitive = "Decimal")]
pub struct SelectorNewtype {}

///
/// NewtypeValidated
///

#[newtype(
    primitive = "Decimal",
    item(prim = "Decimal"),
    ty(validator(path = "validator::num::Lte", args(5.0)))
)]
pub struct NewtypeValidated {}
