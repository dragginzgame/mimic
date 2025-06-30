use crate::{core::traits::ValidatorString, design::prelude::*};

///
/// Kebab
///

#[validator]
pub struct Kebab {}

impl ValidatorString for Kebab {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        if s.is_case(Case::Kebab) {
            Ok(())
        } else {
            Err(format!("'{s}' is not kebab-case"))
        }
    }
}

///
/// Lower
///

#[validator]
pub struct Lower {}

impl ValidatorString for Lower {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        if s.is_case(Case::Lower) {
            Ok(())
        } else {
            Err(format!("'{s}' is not lower case"))
        }
    }
}

///
/// LowerUscore
///

#[validator]
pub struct LowerUscore {}

impl ValidatorString for LowerUscore {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        if s.chars().all(|c| c.is_lowercase() || c == '_') {
            Ok(())
        } else {
            Err(format!("'{s}' is not lower case with_underscores"))
        }
    }
}

///
/// Snake
///

#[validator]
pub struct Snake {}

impl ValidatorString for Snake {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        if s.is_case(Case::Snake) {
            Ok(())
        } else {
            Err(format!("'{s}' is not snake_case"))
        }
    }
}

///
/// Title
///

#[validator]
pub struct Title {}

impl ValidatorString for Title {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        if s.is_case(Case::Title) {
            Ok(())
        } else {
            Err(format!("'{s}' Is Not Title Case"))
        }
    }
}

///
/// Upper
///

#[validator]
pub struct Upper {}

impl ValidatorString for Upper {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        if s.is_case(Case::Upper) {
            Ok(())
        } else {
            Err(format!("'{s}' is not UPPER CASE"))
        }
    }
}
