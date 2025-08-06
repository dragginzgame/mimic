use crate::prelude::*;

///
/// Indexable
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    index(store = "TestIndexStore", fields = "pid, ulid, score"),
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "pid", value(item(prim = "Principal"))),
        field(name = "ulid", value(item(prim = "Ulid"))),
        field(name = "score", value(item(prim = "Nat32"))),
    )
)]
pub struct Indexable {}

///
/// NotIndexable
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "pid", value(item(prim = "Principal"))),
        field(name = "ulid", value(item(prim = "Ulid"))),
        field(name = "score", value(item(prim = "Nat32"))),
    )
)]
pub struct NotIndexable {}
