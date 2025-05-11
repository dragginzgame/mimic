use mimic::{
    orm::base::{types, validator},
    prelude::*,
};

///
/// List
///

#[list(item(is = "types::Text"))]
pub struct List {}

///
/// Map
///

#[map(key(is = "types::Text"), value(item(is = "types::Nat8")))]
pub struct Map {}

///
/// Set
///

#[set(item(is = "types::Text"))]
pub struct Set {}

///
/// ListValidated
///

#[list(item(
    is = "types::Nat8",
    validator(path = "validator::number::Lt", args(10))
))]
pub struct ListValidated {}

///
/// MapValidated
///

#[map(
    key(
        is = "types::Nat8",
        validator(path = "validator::number::Lt", args(10))
    ),
    value(item(
        is = "types::Nat8",
        validator(path = "validator::number::Lt", args(10))
    ))
)]
pub struct MapValidated {}

///
/// SetValidated
///

#[set(item(
    is = "types::Nat8",
    validator(path = "validator::number::Lt", args(10))
))]
pub struct SetValidated {}
