use crate::prelude::*;

///
/// Filterable
///

#[entity(
    store = "crate::schema::FixtureStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(item(prim = "Text"))),
        field(name = "category", value(item(prim = "Text"))),
        field(name = "active", value(item(prim = "Bool"))),
        field(name = "score", value(item(prim = "Float64"))),
        field(name = "level", value(item(prim = "Nat8"))),
        field(name = "offset", value(item(prim = "Int32"))),
        field(name = "tags", value(many, item(prim = "Text"))),
        field(name = "pid", value(item(prim = "Principal"))),
    )
)]
pub struct Filterable {}

impl Filterable {
    #[must_use]
    pub const fn dummy_principal(n: u8) -> Principal {
        Principal::from_slice(&[n; 29])
    }
}
