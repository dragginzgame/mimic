use crate::prelude::*;

///
/// ValidateTest
///

#[entity(
    store = "crate::Store",
    sk(entity = "ValidateTest", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "multiple_ten", value(item(is = "MultipleTenType"))),
    field(
        name = "ltoe_ten",
        value(item(prim = "Nat8", validator(path = "validator::number::Ltoe", args(10)))),
    ),
    field(
        name = "gt_fifty",
        value(item(prim = "Nat8", validator(path = "validator::number::Gt", args(50)))),
    )
)]
pub struct ValidateTest {}

///
/// MultipleTenType
///

#[newtype(
    primitive = "Int32",
    item(prim = "Int32"),
    ty(validator(path = "validator::number::MultipleOf", args(10)))
)]
pub struct MultipleTenType {}
