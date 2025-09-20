use crate::design::{
    base::{sanitizer, validator},
    prelude::*,
};

///
/// CountryCode
/// two-letter country codes defined in ISO 3166-1
///
/// https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
///

#[newtype(
    primitive = "Text",
    item(prim = "Text"),
    ty(
        validator(path = "validator::intl::iso::Iso3166_1A2"),
        sanitizer(path = "sanitizer::intl::iso::Iso3166_1A2"),
    )
)]
pub struct CountryCode {}

///
/// LanguageCode
/// two letter language code
///
/// https://en.wikipedia.org/wiki/ISO_639-1
///

#[newtype(
    primitive = "Text",
    item(prim = "Text"),
    ty(
        validator(path = "validator::intl::iso::Iso639_1"),
        sanitizer(path = "sanitizer::intl::iso::Iso639_1"),
    )
)]
pub struct LanguageCode {}

///
/// PhoneNumber
/// standardised international phone number
///
/// https://en.wikipedia.org/wiki/E.164
///

#[newtype(
    primitive = "Text",
    item(prim = "Text"),
    ty(
        validator(path = "validator::intl::phone::E164PhoneNumber"),
        sanitizer(path = "sanitizer::intl::phone::E164PhoneNumber")
    )
)]
pub struct PhoneNumber {}
