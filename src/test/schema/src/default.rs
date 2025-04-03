use mimic::orm::{base::types, prelude::*};

///
/// Record
///

#[record(
    fields(
        field(name = "nat8_value", value(item(is = "types::Nat8")), default = 1u8),
        field(
            name = "nat8_static_fn",
            value(item(is = "types::Nat8")),
            default = "crate::default::Record::nat8_static_fn"
        ),
    ),
    traits(add(Default))
)]
pub struct Record {}

impl Record {
    #[must_use]
    pub const fn nat8_static_fn() -> u8 {
        32
    }
}
