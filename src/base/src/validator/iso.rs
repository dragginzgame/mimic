pub use crate::prelude::*;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("invalid ISO 3166-1 alpha-2 country code"))]
    InvalidIso6391Code,
}

///
/// Iso6391
///
/// country code
/// https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
///

#[validator]
pub struct Iso6391 {}

impl Iso6391 {
    pub fn validate(code: &str) -> Result<(), Error> {
        if code.len() != 2 || !code.chars().all(|c| c.is_ascii_lowercase()) {
            Err(Error::InvalidIso6391Code)
        } else {
            Ok(())
        }
    }
}
