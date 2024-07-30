use crate::{prelude::*, types, validator};

///
/// Code
/// ISO 639-1 standard language codes
///

#[newtype(
    primitive = "String",
    value(item(is = "types::String")),
    validator(path = "validator::iso::Iso6391")
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
            assert_eq!(
                code.validate().is_ok(),
                expected,
                "testing: {key} - expected: {expected}",
            );
        }
    }
}
