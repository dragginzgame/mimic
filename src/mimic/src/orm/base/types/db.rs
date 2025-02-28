use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// UlidGenerate
///

#[newtype(
    primitive = "Ulid",
    item(is = "types::Ulid"),
    default = "types::Ulid::generate",
    traits(add(SortKey))
)]
pub struct UlidGenerate {}
