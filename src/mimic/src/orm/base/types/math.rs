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

#[newtype(
    primitive = "Decimal",
    item(is = "types::Decimal"),
    traits(remove(ValidateManual))
)]
pub struct DecimalFormat<const I: usize, const F: usize> {}

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum DecimalFormatError {
    #[error("integer length {0} exceeds maximum {1}")]
    IntegerLengthExceeded(usize, usize),

    #[error("fractional length {0} exceeds maximum {1}")]
    FractionalLengthExceeded(usize, usize),
}

impl<const I: usize, const F: usize> ValidateManual for DecimalFormat<I, F> {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        let (ilen, flen) = self.0.count_digits();

        // integer part I
        if ilen > I {
            return Err(DecimalFormatError::IntegerLengthExceeded(ilen, I).into());
        }

        // fractional part F
        if flen > F {
            return Err(DecimalFormatError::FractionalLengthExceeded(flen, F).into());
        }

        Ok(())
    }
}

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
