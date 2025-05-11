use crate::{
    orm::base::{types, validator},
    prelude::*,
};

///
/// Rgb
///

#[record(
    fields(
        field(name = "r", value(item(is = "types::Nat8"))),
        field(name = "g", value(item(is = "types::Nat8"))),
        field(name = "b", value(item(is = "types::Nat8")))
    ),
    traits(add(Default))
)]
pub struct Rgb {}

impl Rgb {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<&RgbHex> for Rgb {
    fn from(hex: &RgbHex) -> Self {
        let r = u8::from_str_radix(&hex.0[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex.0[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex.0[4..6], 16).unwrap_or(0);

        Self { r, g, b }
    }
}

impl TryFrom<&str> for Rgb {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let hex = normalize_rgb_hex(s);
        if hex.len() != 6 {
            return Err("invalid RGB hex length");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "bad red")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "bad green")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "bad blue")?;

        Ok(Self { r, g, b })
    }
}

///
/// Rgba
///

#[record(
    fields(
        field(name = "r", value(item(is = "types::Nat8"))),
        field(name = "g", value(item(is = "types::Nat8"))),
        field(name = "b", value(item(is = "types::Nat8"))),
        field(name = "a", value(item(is = "types::Nat8")))
    ),
    traits(add(Default))
)]
pub struct Rgba {}

impl Rgba {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<&RgbaHex> for Rgba {
    fn from(hex: &RgbaHex) -> Self {
        let r = u8::from_str_radix(&hex.0[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex.0[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex.0[4..6], 16).unwrap_or(0);
        let a = u8::from_str_radix(&hex.0[6..8], 16).unwrap_or(0);

        Self { r, g, b, a }
    }
}

impl TryFrom<&str> for Rgba {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let hex = normalize_rgb_hex(s);
        if hex.len() != 8 {
            return Err("invalid RGBA hex length");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "bad red")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "bad green")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "bad blue")?;
        let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| "bad alpha")?;

        Ok(Self { r, g, b, a })
    }
}

///
/// RgbHex
///

#[newtype(
    primitive = "Text",
    item(is = "types::Text"),
    default = "FFFFFF",
    ty(validator(path = "validator::text::color::RgbHex")),
    traits(remove(From))
)]
pub struct RgbHex {}

impl From<&str> for RgbHex {
    fn from(s: &str) -> Self {
        Self(normalize_rgb_hex(s))
    }
}

impl From<Rgb> for RgbHex {
    fn from(rgb: Rgb) -> Self {
        Self(format!("{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b))
    }
}

///
/// RgbaHex
///

#[newtype(
    primitive = "Text",
    item(is = "types::Text"),
    default = "FFFFFFFF",
    ty(validator(path = "validator::text::color::RgbaHex")),
    traits(remove(From))
)]
pub struct RgbaHex {}

impl From<&str> for RgbaHex {
    fn from(s: &str) -> Self {
        Self(normalize_rgba_hex(s))
    }
}

impl From<Rgba> for RgbaHex {
    fn from(rgba: Rgba) -> Self {
        Self(format!(
            "{:02X}{:02X}{:02X}{:02X}",
            rgba.r, rgba.g, rgba.b, rgba.a
        ))
    }
}

//
// helper functions
//

fn normalize_rgba_hex(input: &str) -> String {
    let hex = input.trim_start_matches('#').to_ascii_uppercase();
    match hex.len() {
        6 => format!("{hex}FF"),
        8 => hex,
        _ => String::from("FFFFFFFF"), // fallback default
    }
}

fn normalize_rgb_hex(input: &str) -> String {
    let hex = input.trim_start_matches('#').to_ascii_uppercase();
    if hex.len() == 6 {
        hex
    } else {
        String::from("FFFFFF") // fallback
    }
}
