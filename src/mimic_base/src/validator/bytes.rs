use mimic::orm::prelude::*;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("Invalid UTF-8 data"))]
    InvalidUTF8,
}

///
/// Utf8
///

#[validator]
pub struct Utf8 {}

impl Utf8 {
    pub fn validate(bytes: &[u8]) -> Result<(), Error> {
        std::str::from_utf8(bytes)
            .map(|_| ())
            .map_err(|_| Error::InvalidUTF8)
    }
}
