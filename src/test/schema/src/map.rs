use mimic::orm::{base::types, prelude::*};

///
/// Map
///

#[map(key(is = "types::String"), value(item(is = "types::U8")))]
pub struct Map {}
