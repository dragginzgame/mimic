#![allow(clippy::cast_possible_wrap)]
pub mod case;

use candid::CandidType;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Display;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("string contains non-ascii characters"))]
    NonAscii,

    #[snafu(display("string has more than {max} repeated consecutive characters"))]
    MaxCharRepeatExceeded { max: isize },

    #[snafu(display("case: {source}"))]
    Case { source: case::Error },
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
