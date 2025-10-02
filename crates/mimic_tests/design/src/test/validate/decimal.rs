use crate::prelude::*;

///
/// ValidateTest
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "multiple_ten", value(item(is = "MultipleTenType"))),
        field(
            name = "lte_ten",
            value(item(prim = "Nat8", validator(path = "validator::num::Lte", args(10)))),
        ),
        field(
            name = "gt_fifty",
            value(item(prim = "Nat8", validator(path = "validator::num::Gt", args(50)))),
        )
    )
)]
pub struct ValidateTest {}

///
/// MultipleTenType
///

#[newtype(
    primitive = "Int32",
    item(prim = "Int32"),
    ty(validator(path = "validator::num::MultipleOf", args(10)))
)]
pub struct MultipleTenType {}

///
/// DecimalMaxDp
///

#[newtype(
    primitive = "Decimal",
    item(prim = "Decimal"),
    ty(validator(path = "validator::decimal::MaxDecimalPlaces", args(3)))
)]
pub struct DecimalMaxDp {}
