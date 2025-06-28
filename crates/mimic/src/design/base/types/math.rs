use crate::design::{base::validator, prelude::*};

///
/// Degrees (°)
///

#[newtype(
    primitive = "Nat16",
    item(prim = "Nat16"),
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
    item(prim = "Nat8"),
    ty(validator(path = "validator::number::Range", args(0, 100)))
)]
pub struct Percent {}

///
/// PercentModifier
///

#[newtype(
    primitive = "Nat16",
    item(prim = "Nat16"),
    ty(validator(path = "validator::number::Range", args(0_u16, 10_000_u16)))
)]
pub struct PercentModifier {}
