use mimic::orm::{
    base::{sanitizer, types},
    prelude::*,
};

///
/// ClampRecord
///

#[record(fields(field(name = "value", value(item(is = "U8Clamp")))))]
pub struct ClampRecord {}

impl ClampRecord {
    #[must_use]
    pub fn new(value: u8) -> Self {
        Self {
            value: value.into(),
        }
    }
}

///
/// U8Clamp
///
/// A U8 value that's always clamped between 10 and 20
///

#[newtype(
    primitive = "U8",
    value(item(is = "types::U8")),
    sanitizer(path = "sanitizer::number::Clamp", args(10_u8, 20_u8))
)]
pub struct U8Clamp {}
