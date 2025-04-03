use crate::orm::prelude::*;

///
/// Url
///

#[validator]
pub struct Url {}

impl ValidatorString for Url {
    fn validate<S: ToString>(&self, s: &S) -> Result<(), String> {
        let s = s.to_string();

        // Very basic check — can be expanded
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(())
        } else {
            Err(format!("URL '{s}' must start with 'http://' or 'https://'"))
        }
    }
}
