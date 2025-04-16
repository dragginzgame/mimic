pub mod case;
pub mod color;
pub mod iso;
pub mod len;

use crate::orm::prelude::*;

///
/// Ascii
///

#[validator]
pub struct Ascii {}

impl ValidatorString for Ascii {
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
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

impl ValidatorString for Version {
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
        match semver::Version::parse(&s.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("invalid version {e}")),
        }
    }
}
