use mimic::orm::{base::types, prelude::*};

///
/// Map
///

#[map(item(is = "Entry"), key = "key")]
pub struct Map {}

///
/// Entry
///

#[record(fields(
    field(name = "key", value(item(is = "types::String"))),
    field(name = "value", value(item(is = "types::U8"))),
))]
pub struct Entry {}

impl Entry {
    #[must_use]
    pub fn new(key: &str, value: u8) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }
}
