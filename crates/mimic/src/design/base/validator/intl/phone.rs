use crate::{core::traits::Validator, design::prelude::*};
use phonenumber::{Mode, parse};

///
/// E164PhoneNumber
/// Ensures phone number is valid and E.164 compliant
///

#[validator]
pub struct E164PhoneNumber;

impl Validator<str> for E164PhoneNumber {
    fn validate(&self, s: &str) -> Result<(), String> {
        match parse(None, s) {
            Ok(num) if num.is_valid() => {
                let e164 = num.format().mode(Mode::E164).to_string();
                if e164.len() > 16 {
                    Err(format!("phone number too long for E.164: {s}"))
                } else {
                    Ok(())
                }
            }
            Ok(_) => Err(format!("invalid phone number: {s}")),
            Err(e) => Err(format!("failed to parse phone number {s}: {e}")),
        }
    }
}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::traits::Validator;

    // -------------------------------------------------------------------------
    // E.164 phone numbers
    // -------------------------------------------------------------------------
    #[test]
    fn test_e164_valid() {
        let v = E164PhoneNumber {};
        assert!(v.validate("+13108675309").is_ok()); // US number
        assert!(v.validate("+442071838750").is_ok()); // UK number
        assert!(v.validate("+819012345678").is_ok()); // Japan number
    }

    #[test]
    fn test_e164_invalid_format() {
        let v = E164PhoneNumber {};
        assert!(v.validate("4155552671").is_err()); // missing +
        assert!(v.validate("+9999999999999999").is_err()); // too long (>15 digits)
    }

    #[test]
    fn test_e164_invalid_number() {
        let v = E164PhoneNumber {};
        assert!(v.validate("+0001234567").is_err()); // 000 not a valid country code
    }
}
