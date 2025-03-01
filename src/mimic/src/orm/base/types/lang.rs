use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// Code
/// ISO 639-1 standard language codes
///

#[newtype(
    primitive = "String",
    item(is = "types::String"),
    ty(validator(path = "validator::string::iso::Iso6391"))
)]
pub struct Code {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        let test_cases = [
            // Valid codes
            ("en", true),
            ("de", true),
            // Invalid codes
            ("D", false),
            ("DE", false),
            ("en-us", false),
            ("EN-US", false),
            ("EN-USSR", false),
        ];

        for (key, expected) in test_cases {
            let code = Code(key.into());
            assert!(
                code.validate().is_ok() == expected,
                "testing: '{}' - expected: {}, got: {}",
                key,
                expected,
                code.validate().is_ok()
            );
        }
    }
}
