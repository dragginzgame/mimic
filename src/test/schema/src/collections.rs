use mimic::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// List
///

#[list(item(is = "types::String"))]
pub struct List {}

///
/// Map
///

#[map(key(is = "types::String"), value(item(is = "types::U8")))]
pub struct Map {}

///
/// Set
///

#[set(item(is = "types::String"))]
pub struct Set {}

///
/// ListValidated
///

#[list(item(is = "types::U8", validator(path = "validator::number::Lt", args(10))))]
pub struct ListValidated {}

///
/// MapValidated
///

#[map(
    key(is = "types::U8", validator(path = "validator::number::Lt", args(10))),
    value(item(is = "types::U8", validator(path = "validator::number::Lt", args(10))))
)]
pub struct MapValidated {}

///
/// SetValidated
///

#[set(item(is = "types::U8", validator(path = "validator::number::Lt", args(10))))]
pub struct SetValidated {}
