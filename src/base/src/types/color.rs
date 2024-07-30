use crate::{prelude::*, sanitizer, types, validator};

///
/// Rgb
///

#[record(fields(
    field(name = "r", value(item(is = "types::U8"))),
    field(name = "g", value(item(is = "types::U8"))),
    field(name = "b", value(item(is = "types::U8")))
))]
pub struct Rgb {}

impl Rgb {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

///
/// Rgba
///

#[record(fields(
    field(name = "r", value(item(is = "types::U8"))),
    field(name = "g", value(item(is = "types::U8"))),
    field(name = "b", value(item(is = "types::U8"))),
    field(name = "a", value(item(is = "types::U8")))
))]
pub struct Rgba {}

impl Rgba {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

///
/// RgbHex
///

#[newtype(
    primitive = "String",
    value(item(is = "types::text::Text<6>"), default = "FFFFFF"),
    sanitizer(path = "sanitizer::color::RgbHex"),
    validator(path = "validator::color::RgbHex")
)]
pub struct RgbHex {}

///
/// RgbaHex
///

#[newtype(
    primitive = "String",
    value(item(is = "types::text::Text<8>"), default = "FFFFFFFF"),
    sanitizer(path = "sanitizer::color::RgbaHex"),
    validator(path = "validator::color::RgbaHex")
)]
pub struct RgbaHex {}
