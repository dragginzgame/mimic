use std::fmt::Display;

///
/// CollapseWhitespace
///

#[sanitizer]
pub struct CollapseWhitespace {}

impl CollapseWhitespace {
    pub fn sanitize<S: Display>(s: S) -> String {
        mimic::lib::string::sanitize::collapse_whitespace(s.to_string())
    }
}

///
/// FixMsWord
///

#[sanitizer]
pub struct FixMsWord {}

impl FixMsWord {
    pub fn sanitize<S: Display>(s: S) -> String {
        mimic::lib::string::sanitize::fix_ms_word(s.to_string())
    }
}
