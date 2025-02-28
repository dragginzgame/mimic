use crate::orm::{
    base::{types, validator},
    prelude::*,
};

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
    item(is = "types::text::Text<6>"),
    default = "FFFFFF",
    ty(validator(path = "validator::color::RgbHex"))
)]
pub struct RgbHex {}

///
/// RgbaHex
///

#[newtype(
    primitive = "String",
    item(is = "types::text::Text<8>"),
    default = "FFFFFFFF",
    ty(validator(path = "validator::color::RgbaHex")),
    traits(remove(From))
)]
pub struct RgbaHex {}

impl From<&str> for RgbaHex {
    fn from(s: &str) -> Self {
        // If the input is 6 characters, append "FF" for full alpha
        let hex = if s.len() == 6 {
            format!("{s}FF")
        } else {
            s.to_owned()
        };

        Self(hex.into())
    }
}
