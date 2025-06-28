use crate::prelude::*;

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
/// ListValidated
///

#[list(item(prim = "Nat8", validator(path = "validator::number::Lt", args(10))))]
pub struct ListValidated {}

///
/// MapValidated
///

#[map(
    key(prim = "Nat8", validator(path = "validator::number::Lt", args(10))),
    value(item(prim = "Nat8", validator(path = "validator::number::Lt", args(10))))
)]
pub struct MapValidated {}

///
/// SetValidated
///

#[set(item(prim = "Nat8", validator(path = "validator::number::Lt", args(10))))]
pub struct SetValidated {}
