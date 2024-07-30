pub mod case;
pub mod misc;

use base::sanitizer::string;
use std::fmt::Display;

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
        let s = string::misc::CollapseWhitespace::sanitize(s);
        let s = string::misc::FixMsWord::sanitize(s);

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

        string::misc::CollapseWhitespace::sanitize(s)
    }
}
