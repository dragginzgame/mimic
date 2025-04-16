use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// Url
///

#[newtype(
    primitive = "Text",
    item(is = "types::Text"),
    ty(validator(path = "validator::web::Url"))
)]
pub struct Url {}
