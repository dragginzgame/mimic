use crate::prelude::*;

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
