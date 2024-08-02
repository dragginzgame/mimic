use crate::{canister, types, validator};
use mimic::orm::prelude::*;

///
/// Validator
///

#[entity(
    store = "canister::test::store::Data",
    fields(
        field(name = "guide", value(item(is = "types::test::validate::GuideType"))),
        field(
            name = "multiple_ten",
            value(item(is = "types::test::validate::MultipleTenType"))
        ),
    )
)]
pub struct Validator {}

///
/// GuideType
///

#[newtype(
    primitive = "U8",
    value(item(is = "types::U8")),
    guide(
        entry(name = "Value A", value = 5),
        entry(name = "Value B", value = 6),
        entry(name = "Value C", value = 7),
    )
)]
pub struct GuideType {}

///
/// MultipleTenType
///

#[newtype(
    primitive = "I32",
    value(item(is = "types::I32")),
    validator(path = "validator::number::MultipleOf", args(10))
)]
pub struct MultipleTenType {}
