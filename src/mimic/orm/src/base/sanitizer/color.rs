use crate::prelude::*;

///
/// RgbHex
///

#[sanitizer]
pub struct RgbHex {}

impl RgbHex {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string().to_uppercase()
    }
}

///
/// RgbaHex
///

#[sanitizer]
pub struct RgbaHex {}

impl RgbaHex {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        let s = s.to_string();

        if s.len() == 6 { format!("{s}FF") } else { s }.to_uppercase()
    }
}
