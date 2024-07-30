use mimic::lib::case::{Case, Casing};
use std::fmt::Display;

///
/// Camel
///

#[sanitizer]
pub struct Camel {}

impl Camel {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string().to_case(Case::Camel)
    }
}

///
/// Kebab
///

#[sanitizer]
pub struct Kebab {}

impl Kebab {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string().to_case(Case::Kebab)
    }
}

///
/// Lower
///

#[sanitizer]
pub struct Lower {}

impl Lower {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string().to_case(Case::Lower)
    }
}

///
/// Snake
///

#[sanitizer]
pub struct Snake {}

impl Snake {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string().to_case(Case::Snake)
    }
}

///
/// Title
///

#[sanitizer]
pub struct Title {}

impl Title {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string().to_case(Case::Title)
    }
}

///
/// Upper
///

#[sanitizer]
pub struct Upper {}

impl Upper {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string().to_case(Case::Upper)
    }
}

///
/// UpperCamel
///

#[sanitizer]
pub struct UpperCamel {}

impl UpperCamel {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string().to_case(Case::UpperCamel)
    }
}

///
/// UpperSnake
///

#[sanitizer]
pub struct UpperSnake {}

impl UpperSnake {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string().to_case(Case::UpperSnake)
    }
}

///
/// UpperKebab
///

#[sanitizer]
pub struct UpperKebab {}

impl UpperKebab {
    #[must_use]
    pub fn sanitize<S: Display>(s: S) -> String {
        s.to_string().to_case(Case::UpperKebab)
    }
}
