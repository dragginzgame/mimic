use mimic::orm::{base::types, prelude::*};

///
/// HasMap
///

#[entity(
    store = "crate::Store",
    sk(entity = "HasMap", field = "id"),
    fields(
        field(
            name = "id",
            value(item(is = "types::Ulid"), default = "types::Ulid::generate")
        ),
        field(name = "map_int_string", value(item(is = "MapIntString")))
    )
)]
pub struct HasMap {}

///
/// MapIntString
///

#[map(key(is = "types::I32"), value(item(is = "types::String")))]
pub struct MapIntString {}
