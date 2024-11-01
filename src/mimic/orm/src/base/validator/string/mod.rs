#![allow(clippy::cast_possible_wrap)]
pub mod case;

use crate::prelude::*;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("string contains non-ascii characters"))]
    NonAscii,

    #[snafu(display("{error}"))]
    InvalidVersion { error: String },
}

///
/// Ascii
///

#[validator]
pub struct Ascii {}

impl Ascii {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        if d.to_string().is_ascii() {
            Ok(())
        } else {
            Err(Error::NonAscii)
        }
    }
}

///
/// Version
/// (semver crate)
///

#[validator]
pub struct Version {}

impl Version {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        match semver::Version::parse(&d.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::InvalidVersion {
                error: e.to_string(),
            }),
        }
    }
}
