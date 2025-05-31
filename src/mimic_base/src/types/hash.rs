use crate::prelude::*;

///
/// Sha256
///

#[newtype(
    primitive = "Text",
    item(is = "types::Text"),
    ty(validator(path = "validator::hash::Sha256"))
)]
pub struct Sha256 {}
