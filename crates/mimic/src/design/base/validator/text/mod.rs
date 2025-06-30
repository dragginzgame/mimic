pub mod case;
pub mod color;
pub mod iso;
pub mod len;

use crate::{core::traits::ValidatorString, design::prelude::*};

///
/// AlphaUscore
/// this doesn't force ASCII, instead we're using the unicode is_alphabetic
/// and ASCII is handled in a separate validator
///

#[validator]
pub struct AlphaUscore {}

impl ValidatorString for AlphaUscore {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        if s.chars().all(|c| c.is_alphabetic() || c == '_') {
            Ok(())
        } else {
            Err(format!("'{s}' is not alphabetic with underscores"))
        }
    }
}

///
/// AlphanumUscore
///

#[validator]
pub struct AlphanumUscore {}

impl ValidatorString for AlphanumUscore {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        if s.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Ok(())
        } else {
            Err(format!("'{s}' is not alphanumeric with underscores"))
        }
    }
}

///
/// Ascii
///

#[validator]
pub struct Ascii {}

impl ValidatorString for Ascii {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        if s.as_ref().is_ascii() {
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
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        match semver::Version::parse(s.as_ref()) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("invalid version {e}")),
        }
    }
}
