use crate::prelude::*;

///
/// Index
///

#[entity(
    store = "crate::Store",
    sk(entity = "Index", field = "id"),
    index(fields = "x", store = "crate::Index"),
    index(fields = "y", store = "crate::Index", unique),
    field(
        name = "id",
        value(item(is = "types::Ulid")),
        default = "types::Ulid::generate"
    ),
    field(name = "x", value(item(is = "types::Nat32"))),
    field(name = "y", value(item(is = "types::Nat32")))
)]
pub struct Index {}
