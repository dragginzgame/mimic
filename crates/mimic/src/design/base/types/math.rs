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

///
/// DecimalRange
///

#[record(
    fields(
        field(name = "min", value(item(prim = "Decimal"))),
        field(name = "max", value(item(prim = "Decimal"))),
    ),
    traits(remove(ValidateCustom))
)]
pub struct DecimalRange {}

impl DecimalRange {
    #[must_use]
    pub fn new(min: Decimal, max: Decimal) -> Self {
        Self { min, max }
    }
}

impl ValidateCustom for DecimalRange {
    fn validate_custom(&self) -> Result<(), ErrorTree> {
        validator::number::Lte::new(self.max)
            .validate(&self.min)
            .map_err(ErrorTree::from)
    }
}

///
/// Int32Range
///

#[record(
    fields(
        field(name = "min", value(item(prim = "Int32"))),
        field(name = "max", value(item(prim = "Int32"))),
    ),
    traits(remove(ValidateCustom))
)]
pub struct Int32Range {}

impl Int32Range {
    #[must_use]
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

impl ValidateCustom for Int32Range {
    fn validate_custom(&self) -> Result<(), ErrorTree> {
        validator::number::Lte::new(self.max)
            .validate(&self.min)
            .map_err(ErrorTree::from)
    }
}

///
/// Nat32Range
///

#[record(
    fields(
        field(name = "min", value(item(prim = "Nat32"))),
        field(name = "max", value(item(prim = "Nat32"))),
    ),
    traits(remove(ValidateCustom))
)]
pub struct Nat32Range {}

impl Nat32Range {
    #[must_use]
    pub fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
}

impl ValidateCustom for Nat32Range {
    fn validate_custom(&self) -> Result<(), ErrorTree> {
        validator::number::Lte::new(self.max)
            .validate(&self.min)
            .map_err(ErrorTree::from)
    }
}
