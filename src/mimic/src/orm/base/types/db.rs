use crate::orm::{
    base::{types, validator},
    prelude::*,
    traits::EntityId,
};

///
/// UlidGenerate
///

#[newtype(
    primitive = "Ulid",
    item(is = "Ulid"),
    default = "Ulid::generate",
    traits(add(SortKey))
)]
pub struct UlidGenerate {}
