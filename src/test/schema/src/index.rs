use mimic::orm::{base::types, prelude::*};

///
/// Index
///

#[entity(
    sk(entity = "Index", field = "id"),
    index(fields = "x"),
    index(fields = "y", unique),
    fields(
        field(
            name = "id",
            value(item(is = "types::Ulid"), default = "types::Ulid::generate")
        ),
        field(name = "x", value(item(is = "types::U32"))),
        field(name = "y", value(item(is = "types::U32")))
    )
)]
pub struct Index {}
