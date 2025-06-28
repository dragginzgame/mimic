use crate::design::prelude::*;

///
/// MimeType
///

#[validator]
pub struct MimeType {}

impl ValidatorString for MimeType {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(format!("MIME type '{s}' must contain exactly one '/'"));
        }

        let is_valid_part = |part: &str| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || "+.-".contains(c))
        };

        if !is_valid_part(parts[0]) || !is_valid_part(parts[1]) {
            return Err(format!(
                "MIME type '{s}' contains invalid characters; only alphanumeric, '+', '-', '.' allowed"
            ));
        }

        Ok(())
    }
}

///
/// Url
///

#[validator]
pub struct Url {}

impl ValidatorString for Url {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let s = s.as_ref();

        // Very basic check — can be expanded
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(())
        } else {
            Err(format!("URL '{s}' must start with 'http://' or 'https://'"))
        }
    }
}
