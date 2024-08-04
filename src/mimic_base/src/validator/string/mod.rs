#![allow(clippy::cast_possible_wrap)]
pub mod case;

use mimic::orm::prelude::*;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("string contains non-ascii characters"))]
    NonAscii,
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
