use mimic::{base::types, prelude::*};

///
/// Index
///

#[entity(
    store = "crate::Store",
    sk(entity = "Index", field = "id"),
    index(fields = "x"),
    index(fields = "y", unique),
    fields(
        field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
        field(name = "x", value(item(is = "types::Nat32"))),
        field(name = "y", value(item(is = "types::Nat32")))
    )
)]
pub struct Index {}
