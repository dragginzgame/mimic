use crate::{
    orm::base::{types, validator},
    prelude::*,
};

///
/// DecimalFormat
///

#[newtype(primitive = "Decimal", item(is = "types::Decimal"))]
pub struct DecimalFormat {}

///
/// Degrees (°)
///

#[newtype(
    primitive = "Nat16",
    item(is = "types::Nat16"),
    ty(validator(path = "validator::number::Range", args(0_u16, 360_u16)))
)]
pub struct Degrees {}

///
/// Percent
///
/// basic percentage as an integer
///

#[newtype(
    primitive = "Nat8",
    item(is = "types::Nat8"),
    ty(validator(path = "validator::number::Range", args(0, 100)))
)]
pub struct Percent {}

///
/// PercentModifier
///

#[newtype(
    primitive = "Nat16",
    item(is = "types::Nat16"),
    ty(validator(path = "validator::number::Range", args(0_u16, 10_000_u16)))
)]
pub struct PercentModifier {}
