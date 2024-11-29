use mimic::orm::{base::types, prelude::*};

///
/// Index
///

#[entity(
    store = "crate::Store",
    sk(entity = "Index", field = "id"),
    index(fields = "x"),
    fields(
        field(name = "id", value(item(is = "types::Ulid"))),
        field(name = "x", value(item(is = "types::U32")))
    )
)]
pub struct Index {}
