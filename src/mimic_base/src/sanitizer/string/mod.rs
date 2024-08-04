pub mod case;

use crate::sanitizer::string;
use mimic::orm::prelude::*;

///
/// CollapseWhitespace
///

#[sanitizer]
pub struct CollapseWhitespace {}

impl CollapseWhitespace {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

///
/// Title
/// formats with title case, and strips and collapses whitespace
///

#[sanitizer]
pub struct Title {}

impl Title {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        let s = s.to_string();
        let s = CollapseWhitespace::sanitize(s);

        string::case::Title::sanitize(s)
    }
}

///
/// Paragraph
/// for general formatted text (descriptions, flavour text)
///

#[sanitizer]
pub struct Paragraph {}

impl Paragraph {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        let s = s.to_string();

        CollapseWhitespace::sanitize(s)
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
            assert_eq!(
                CollapseWhitespace::sanitize(input),
                *expected,
                "testing: {input}"
            );
        }
    }
}
