use crate::core::traits::Sanitizer;
use mimic::design::prelude::*;
use phonenumber::{Mode, parse};

///
/// E164PhoneNumber
/// Parses and re-formats input into canonical E.164 string
///

#[sanitizer]
pub struct E164PhoneNumber;

impl Sanitizer<String> for E164PhoneNumber {
    fn sanitize(&self, value: String) -> String {
        match parse(None, &value) {
            Ok(num) => num.format().mode(Mode::E164).to_string(),
            // if parsing fails, leave it unchanged (so validator will catch it)
            Err(_) => value,
        }
    }
}
