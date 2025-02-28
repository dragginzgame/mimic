use mimic::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// ValidateTest
///

#[entity(
    store = "crate::Store",
    sk(entity = "ValidateTest", field = "id"),
    fields(
        field(name = "id", value(item(is = "types::db::UlidGenerate"))),
        field(name = "multiple_ten", value(item(is = "MultipleTenType")))
    )
)]
pub struct ValidateTest {}

///
/// MultipleTenType
///

#[newtype(
    primitive = "I32",
    item(is = "types::I32"),
    ty(validator(path = "validator::number::MultipleOf", args(10)))
)]
pub struct MultipleTenType {}
