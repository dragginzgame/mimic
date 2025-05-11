use crate::{
    orm::base::{types, validator},
    prelude::*,
};

///
/// Function
///
/// 30 characters, snake_case
///

#[newtype(
    primitive = "Text",
    item(is = "types::Text"),
    ty(
        validator(path = "validator::text::len::Range", args(3, 30)),
        validator(path = "validator::text::case::Snake"),
    ),
    traits(add(Hash))
)]
pub struct Function {}
