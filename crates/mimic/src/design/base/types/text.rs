use crate::design::{base::validator, prelude::*};

///
/// Function
///
/// 30 characters, snake_case
///

#[newtype(
    primitive = "Text",
    item(prim = "Text"),
    ty(
        validator(path = "validator::text::len::Range", args(3, 30)),
        validator(path = "validator::text::case::Snake"),
    )
)]
pub struct Function {}
