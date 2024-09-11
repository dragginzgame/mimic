use mimic::orm::prelude::*;
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("RGB string '{hex}' should be 6 hexadecimal characters"))]
    InvalidRgbHex { hex: String },

    #[snafu(display("RGBA string '{hex}' should be 8 hexadecimal characters"))]
    InvalidRgbaHex { hex: String },
}

///
/// RgbHex
///

#[validator]
pub struct RgbHex {}

impl RgbHex {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        let hex = d.to_string();

        if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(())
        } else {
            Err(Error::InvalidRgbHex { hex })
        }
    }
}

///
/// RgbaHex
///

#[validator]
pub struct RgbaHex {}

impl RgbaHex {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        let hex = d.to_string();

        if hex.len() == 8 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(())
        } else {
            Err(Error::InvalidRgbaHex { hex })
        }
    }
}
