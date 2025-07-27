use crate::design::{base::validator, prelude::*};

///
/// Ident
///

#[newtype(
    primitive = "Text",
    item(prim = "Text"),
    ty(
        validator(path = "validator::text::len::Range", args(2, 30)),
        validator(path = "validator::text::case::Snake"),
    )
)]
pub struct Ident {}

///
/// Function
///
/// 30 characters, snake_case
///

#[newtype(
    primitive = "Text",
    item(prim = "Text"),
    ty(
        validator(path = "validator::text::len::Range", args(2, 64)),
        validator(path = "validator::text::case::Snake"),
    )
)]
pub struct Function {}
