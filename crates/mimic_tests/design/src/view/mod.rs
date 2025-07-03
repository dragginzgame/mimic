use crate::prelude::*;

///
/// DecimalMaxDp
///

#[newtype(
    primitive = "Decimal",
    item(prim = "Decimal"),
    ty(validator(path = "validator::decimal::MaxDecimalPlaces", args(3)))
)]
pub struct DecimalMaxDp {}
