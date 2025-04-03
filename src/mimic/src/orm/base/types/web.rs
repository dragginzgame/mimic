use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// Url
///

#[newtype(
    primitive = "String",
    item(is = "types::String"),
    ty(validator(path = "validator::web::Url"))
)]
pub struct Url {}
