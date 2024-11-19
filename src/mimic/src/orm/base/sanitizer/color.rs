use crate::orm::prelude::*;

///
/// RgbHex
///

#[sanitizer]
pub struct RgbHex {}

impl Sanitizer for RgbHex {
    fn sanitize_string<S: ToString>(&self, s: &S) -> String {
        s.to_string().to_uppercase()
    }
}

///
/// RgbaHex
///

#[sanitizer]
pub struct RgbaHex {}

impl Sanitizer for RgbaHex {
    fn sanitize_string<S: ToString>(&self, s: &S) -> String {
        let s = s.to_string();

        if s.len() == 6 { format!("{s}FF") } else { s }.to_uppercase()
    }
}
