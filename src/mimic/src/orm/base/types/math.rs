use crate::{
    Error, ThisError,
    orm::{
        base::{types, validator},
        prelude::*,
    },
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
    primitive = "U16",
    item(is = "types::U16"),
    ty(validator(path = "validator::number::Range", args(0, 360)))
)]
pub struct Degrees {}

///
/// Percent
///
/// basic percentage as an integer
///

#[newtype(
    primitive = "U8",
    item(is = "types::U8"),
    ty(validator(path = "validator::number::Range", args(0, 100)))
)]
pub struct Percent {}

///
/// PercentModifier
///

#[newtype(
    primitive = "U16",
    item(is = "types::U16"),
    ty(validator(path = "validator::number::Range", args(0, 10_000)))
)]
pub struct PercentModifier {}
