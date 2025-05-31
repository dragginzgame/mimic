use crate::prelude::*;

///
/// MimeType
///

#[newtype(
    primitive = "Text",
    item(is = "types::Text"),
    ty(validator(path = "validator::web::MimeType"))
)]
pub struct MimeType {}

///
/// Url
///

#[newtype(
    primitive = "Text",
    item(is = "types::Text"),
    ty(validator(path = "validator::web::Url"))
)]
pub struct Url {}
