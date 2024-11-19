use crate::orm::prelude::*;

///
/// Utf8
///

#[validator]
pub struct Utf8 {}

impl Validator for Utf8 {
    fn validate_blob(&self, bytes: &[u8]) -> Result<(), String> {
        std::str::from_utf8(bytes)
            .map(|_| ())
            .map_err(|_| "invalid utf-8 data".to_string())
    }
}
