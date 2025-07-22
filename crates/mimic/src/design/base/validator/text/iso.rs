use crate::{core::traits::Validator, design::prelude::*};

///
/// Iso6391
///
/// country code
/// https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
///

#[validator]
pub struct Iso6391 {}

impl Validator<str> for Iso6391 {
    fn validate(&self, s: &str) -> Result<(), String> {
        if s.len() != 2 || !s.chars().all(|c| c.is_ascii_lowercase()) {
            Err(format!("invalid ISO 3166-1 alpha-2 country code {s}"))
        } else {
            Ok(())
        }
    }
}
