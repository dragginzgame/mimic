pub(crate) mod prelude {
    pub use mimic::design::{
        base::{types, validator},
        prelude::*,
    };
}
pub use prelude::*;

///
/// DecimalMaxDp
///

#[newtype(
    primitive = "Decimal",
    item(prim = "Decimal"),
    ty(validator(path = "validator::decimal::MaxDecimalPlaces", args(3)))
)]
pub struct DecimalMaxDp {}

///
/// Float32
///

#[newtype(
    primitive = "Float32",
    item(prim = "Float32"),
    ty(validator(path = "validator::number::Ltoe", args(5.0)))
)]
pub struct Float32 {}

///
/// Float64
///

#[newtype(
    primitive = "Float64",
    item(prim = "Float64"),
    ty(validator(path = "validator::number::Ltoe", args(5.0)))
)]
pub struct Float64 {}
