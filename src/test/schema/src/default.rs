use mimic::orm::{base::types, prelude::*};

///
/// Record
///

#[record(
    fields(
        field(name = "u8_value", value(item(is = "types::U8")), default = 1u8),
        field(
            name = "u8_static_fn",
            value(item(is = "types::U8")),
            default = "crate::default::Record::u8_static_fn"
        ),
    ),
    traits(add(Default))
)]
pub struct Record {}

impl Record {
    #[must_use]
    pub const fn u8_static_fn() -> u8 {
        32
    }
}
