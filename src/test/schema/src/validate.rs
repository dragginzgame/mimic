use base::{types, validator};
use mimic::orm::prelude::*;

///
/// Validator
///

#[entity(
    store = "crate::Store",
    fields(field(name = "multiple_ten", value(item(is = "MultipleTenType"))))
)]
pub struct Validator {}

///
/// MultipleTenType
///

#[newtype(
    primitive = "I32",
    value(item(is = "types::I32")),
    validator(path = "validator::number::MultipleOf", args(10))
)]
pub struct MultipleTenType {}
