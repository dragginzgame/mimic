use crate::prelude::*;

///
/// Entity
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(ident = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(ident = "a", value(item(prim = "Int32")), default = 3),
    )
)]
pub struct Entity {}

///
/// UnitKey
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(ident = "id", value(item(prim = "Unit"))),
        field(ident = "a", value(item(prim = "Int32")), default = 3),
    )
)]
pub struct UnitKey {}
