use crate::design::prelude::*;

///
/// Sha256
///

#[validator]
pub struct Sha256 {}

impl ValidatorString for Sha256 {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        // len
        if s.len() != 64 {
            return Err(format!("must be 64 characters, got {}", s.len()));
        }

        // hex
        if !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("must contain only hexadecimal characters (0-9, a-f)".to_string());
        }

        Ok(())
    }
}
