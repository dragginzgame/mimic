use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// Function
///
/// 30 characters, snake_case
///

#[newtype(
    primitive = "String",
    item(is = "types::String"),
    ty(
        validator(path = "validator::string::len::Range", args(3, 30)),
        validator(path = "validator::string::case::Snake"),
    ),
    traits(add(Hash))
)]
pub struct Function {}
