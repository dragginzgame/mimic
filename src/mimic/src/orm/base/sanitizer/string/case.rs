use crate::orm::prelude::*;

///
/// Camel
///

#[sanitizer]
pub struct Camel {}

impl Sanitizer for Camel {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        Ok(s.to_string().to_case(Case::Camel))
    }
}

///
/// Kebab
///

#[sanitizer]
pub struct Kebab {}

impl Sanitizer for Kebab {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        Ok(s.to_string().to_case(Case::Kebab))
    }
}

///
/// Lower
///

#[sanitizer]
pub struct Lower {}

impl Sanitizer for Lower {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        Ok(s.to_string().to_case(Case::Lower))
    }
}

///
/// Snake
///

#[sanitizer]
pub struct Snake {}

impl Sanitizer for Snake {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        Ok(s.to_string().to_case(Case::Snake))
    }
}

///
/// Title
///

#[sanitizer]
pub struct Title {}

impl Sanitizer for Title {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        Ok(s.to_string().to_case(Case::Title))
    }
}

///
/// Upper
///

#[sanitizer]
pub struct Upper {}

impl Sanitizer for Upper {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        Ok(s.to_string().to_case(Case::Upper))
    }
}

///
/// UpperCamel
///

#[sanitizer]
pub struct UpperCamel {}

impl Sanitizer for UpperCamel {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        Ok(s.to_string().to_case(Case::UpperCamel))
    }
}

///
/// UpperSnake
///

#[sanitizer]
pub struct UpperSnake {}

impl Sanitizer for UpperSnake {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        Ok(s.to_string().to_case(Case::UpperSnake))
    }
}

///
/// UpperKebab
///

#[sanitizer]
pub struct UpperKebab {}

impl Sanitizer for UpperKebab {
    fn sanitize_string<S: ToString>(&self, s: &S) -> Result<String, String> {
        Ok(s.to_string().to_case(Case::UpperKebab))
    }
}
