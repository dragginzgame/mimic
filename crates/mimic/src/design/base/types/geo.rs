use crate::design::{base::validator, prelude::*};

///
/// Country
///

#[newtype(
    primitive = "Text",
    item(prim = "Text"),
    ty(validator(path = "validator::intl::iso::Iso3166_1A2"))
)]
pub struct Country {}

///
/// PhoneNumber
///

#[newtype(
    primitive = "Text",
    item(prim = "Text"),
    ty(validator(path = "validator::intl::phone::E164PhoneNumber"))
)]
pub struct PhoneNumber {}
