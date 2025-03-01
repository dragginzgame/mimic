use mimic::orm::{base::types, prelude::*};

///
/// Map
///

#[map(key(is = "types::String"), value(item(is = "types::U8")))]
pub struct Map {}

///
/// Set
///

#[set(item(is = "types::String"))]
pub struct Set {}
