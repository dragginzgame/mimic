pub mod case;

use crate::orm::prelude::*;

///
/// CollapseWhitespace
///

#[sanitizer]
pub struct CollapseWhitespace {}

impl Sanitizer for CollapseWhitespace {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        let s = s
            .to_string()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        Ok(s)
    }
}

///
/// Paragraph
/// for general formatted text (descriptions, flavour text)
///

#[sanitizer]
pub struct Paragraph {}

impl Sanitizer for Paragraph {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        CollapseWhitespace::default().sanitize_string(s)
    }
}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapse_whitespace() {
        let test_cases = [
            ("", ""),
            ("    ", ""),
            ("\n", ""),
            ("\n\n", ""),
            (" \n \n ", ""),
            ("   hello   ", "hello"),
            ("hello   ", "hello"),
            ("   hello", "hello"),
            ("hello world", "hello world"),
            ("  hello   world  ", "hello world"),
            ("hello   world  \n", "hello world"),
            ("hello\tworld", "hello world"),
            ("\t\na\n\n\nc        \t   \t", "a c"),
        ];

        for (input, expected) in &test_cases {
            let cw = CollapseWhitespace::default();

            assert_eq!(
                cw.sanitize_string(input).unwrap(),
                *expected,
                "testing: {input}"
            );
        }
    }
}
