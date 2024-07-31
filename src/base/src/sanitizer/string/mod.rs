pub mod case;

use base::sanitizer::string;
use std::fmt::Display;

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
