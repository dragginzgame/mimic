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
    traits(add(SortKey), remove(From))
)]
pub struct UlidGenerate {}

impl<T: Into<types::Ulid>> From<T> for UlidGenerate {
    fn from(t: T) -> Self {
        Self(t.into())
    }
}
