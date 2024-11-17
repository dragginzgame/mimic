#![allow(clippy::cast_possible_wrap)]
pub mod case;

use crate::orm::prelude::*;

///
/// Ascii
///

#[validator]
pub struct Ascii {}

impl Validator for Ascii {
    fn validate_string<S: ToString>(&self, s: &S) -> Result<(), String> {
        if s.to_string().is_ascii() {
            Ok(())
        } else {
            Err("string contains non-ascii characters".to_string())
        }
    }
}

///
/// Version
/// (semver crate)
///

#[validator]
pub struct Version {}

impl Validator for Version {
    fn validate_string<S: ToString>(&self, s: &S) -> Result<(), String> {
        match semver::Version::parse(&s.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("invalid version {e}")),
        }
    }
}
