use crate::orm::prelude::*;

///
/// RgbHex
///

#[validator]
pub struct RgbHex {}

impl Validator for RgbHex {
    fn validate_string<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

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

impl Validator for RgbaHex {
    fn validate_string<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

        if s.len() == 8 && s.chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(())
        } else {
            Err(format!(
                "RGBA string '{s}' should be 8 hexadecimal characters"
            ))
        }
    }
}
