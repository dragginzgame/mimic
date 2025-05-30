use mimic::{
    base::{types, validator},
    prelude::*,
};

///
/// ValidateTest
///

#[entity(
    store = "crate::Store",
    sk(entity = "ValidateTest", field = "id"),
    field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
    field(name = "multiple_ten", value(item(is = "MultipleTenType"))),
    field(
        name = "ltoe_ten",
        value(item(
            is = "types::Nat8",
            validator(path = "validator::number::Ltoe", args(10))
        )),
    ),
    field(
        name = "gt_fifty",
        value(item(
            is = "types::Nat8",
            validator(path = "validator::number::Gt", args(50))
        )),
    )
)]
pub struct ValidateTest {}

///
/// MultipleTenType
///

#[newtype(
    primitive = "Int32",
    item(is = "types::Int32"),
    ty(validator(path = "validator::number::MultipleOf", args(10)))
)]
pub struct MultipleTenType {}
