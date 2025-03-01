use crate::orm::prelude::*;

///
/// AlphaUscore
/// this doesn't force ASCII, instead we're using the unicode is_alphabetic
/// and ASCII is handled in a separate validator
///

#[validator]
pub struct AlphaUscore {}

impl ValidatorString for AlphaUscore {
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

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
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

        if s.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Ok(())
        } else {
            Err(format!("'{s}' is not alphanumeric with underscores"))
        }
    }
}

///
/// Kebab
///

#[validator]
pub struct Kebab {}

impl ValidatorString for Kebab {
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

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
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

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
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

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
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

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
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

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
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

        if s.is_case(Case::Upper) {
            Ok(())
        } else {
            Err(format!("'{s}' is not UPPER CASE"))
        }
    }
}
