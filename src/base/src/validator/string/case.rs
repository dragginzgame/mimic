use candid::CandidType;
use mimic::lib::case::{Case, Casing};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::fmt::Display;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("'{s}' is not alphabetic with underscores"))]
    NotAlphaUscore { s: String },

    #[snafu(display("'{s}' is not alphanumeric with underscores"))]
    NotAlphanumUscore { s: String },

    #[snafu(display("'{s}' is not kebab-case"))]
    NotKebab { s: String },

    #[snafu(display("'{s}' is not lower case"))]
    NotLower { s: String },

    #[snafu(display("'{s}' is not lower case with underscores"))]
    NotLowerUscore { s: String },

    #[snafu(display("'{s}' is not snake_case"))]
    NotSnake { s: String },

    #[snafu(display("'{s}' is not upper case"))]
    NotUpper { s: String },
}

///
/// AlphaUscore
/// this doesn't force ASCII, instead we're using the unicode is_alphabetic
/// and ASCII is handled in a separate validator
///

#[validator]
pub struct AlphaUscore {}

impl AlphaUscore {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        let s = d.to_string();

        if s.chars().all(|c| c.is_alphabetic() || c == '_') {
            Ok(())
        } else {
            Err(Error::NotAlphaUscore { s })
        }
    }
}

///
/// AlphanumUscore
///

#[validator]
pub struct AlphanumUscore {}

impl AlphanumUscore {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        let s = d.to_string();

        if s.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Ok(())
        } else {
            Err(Error::NotAlphanumUscore { s })
        }
    }
}

///
/// Kebab
///

#[validator]
pub struct Kebab {}

impl Kebab {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        let s = d.to_string();

        if s.is_case(Case::Kebab) {
            Ok(())
        } else {
            Err(Error::NotKebab { s })
        }
    }
}

///
/// Lower
///

#[validator]
pub struct Lower {}

impl Lower {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        let s = d.to_string();

        if s.is_case(Case::Lower) {
            Ok(())
        } else {
            Err(Error::NotLower { s })
        }
    }
}

///
/// LowerUscore
///

#[validator]
pub struct LowerUscore {}

impl LowerUscore {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        let s = d.to_string();

        if s.chars().all(|c| c.is_lowercase() || c == '_') {
            Ok(())
        } else {
            Err(Error::NotLowerUscore { s })
        }
    }
}

///
/// Snake
///

#[validator]
pub struct Snake {}

impl Snake {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        let s = d.to_string();

        if s.is_case(Case::Snake) {
            Ok(())
        } else {
            Err(Error::NotSnake { s })
        }
    }
}

///
/// Upper
///

#[validator]
pub struct Upper {}

impl Upper {
    pub fn validate<D: Display>(d: &D) -> Result<(), Error> {
        let s = d.to_string();

        if s.is_case(Case::Upper) {
            Ok(())
        } else {
            Err(Error::NotUpper { s })
        }
    }
}
