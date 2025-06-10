use crate::prelude::*;

///
/// RgbHex
///

#[validator]
pub struct RgbHex {}

impl ValidatorString for RgbHex {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        if s.len() == 6 && s.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(())
        } else {
            Err(format!(
                "RGBA string '{s}' should be 6 hexadecimal characters"
            ))
        }
    }
}

///
/// RgbaHex
///

#[validator]
pub struct RgbaHex {}

impl ValidatorString for RgbaHex {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        if s.len() == 8 && s.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(())
        } else {
            Err(format!(
                "RGBA string '{s}' should be 8 hexadecimal characters"
            ))
        }
    }
}
