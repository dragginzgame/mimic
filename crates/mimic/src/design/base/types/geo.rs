use crate::design::{base::validator, prelude::*};

///
/// Country
/// ISO 3166-1 country codes
///

#[newtype(
    primitive = "Text",
    item(prim = "Text"),
    ty(validator(path = "validator::iso::Iso3166_1A2"))
)]
pub struct Country {}
