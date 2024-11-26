use crate::orm::prelude::*;

///
/// RgbHex
///

#[sanitizer]
pub struct RgbHex {}

impl Sanitizer for RgbHex {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        Ok(s.to_string().to_uppercase())
    }
}

///
/// RgbaHex
///

#[sanitizer]
pub struct RgbaHex {}

impl Sanitizer for RgbaHex {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        let mut res = s.to_string();

        if res.len() == 6 {
            res.push_str("FF");
        }

        Ok(res.to_uppercase())
    }
}
