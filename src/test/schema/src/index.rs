use crate::prelude::*;

///
/// Index
///

#[entity(
    store = "crate::Store",
    sk(entity = "Index", field = "id"),
    index(store = "crate::Index", fields = "x"),
    index(store = "crate::Index", fields = "y", unique),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "x", value(item(prim = "Nat32"))),
    field(name = "y", value(item(prim = "Nat32")))
)]
pub struct Index {}
