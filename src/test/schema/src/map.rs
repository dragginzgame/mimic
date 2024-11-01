use mimic::orm::{base::types, prelude::*};

///
/// HasMap
///

#[entity(
    store = "crate::Store",
    fields(field(name = "map_int_string", value(item(is = "MapIntString"))))
)]
pub struct HasMap {}

///
/// MapIntString
///

#[map(key(is = "types::I32"), value(item(is = "types::String")))]
pub struct MapIntString {}
