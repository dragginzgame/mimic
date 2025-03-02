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
        field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
        field(name = "multiple_ten", value(item(is = "MultipleTenType"))),
        field(
            name = "ltoe_ten",
            value(item(
                is = "types::U8",
                validator(path = "validator::number::Ltoe", args(10))
            )),
        ),
        field(
            name = "gt_fifty",
            value(item(is = "types::U8", validator(path = "validator::number::Gt", args(50)))),
        ),
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
