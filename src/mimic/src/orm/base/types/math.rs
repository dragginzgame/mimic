use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// DecimalFormat
///

#[newtype(
    primitive = "Decimal",
    value(item(is = "types::Decimal")),
    traits(remove(ValidateManual))
)]
pub struct DecimalFormat<const I: usize, const F: usize> {}

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum DecimalFormatError {
    #[snafu(display("integer length {ilen} exceeds maximum {max}"))]
    IntegerLengthExceeded { ilen: usize, max: usize },

    #[snafu(display("fractional length {flen} exceeds maximum {max}"))]
    FractionalLengthExceeded { flen: usize, max: usize },
}

impl<const I: usize, const F: usize> ValidateManual for DecimalFormat<I, F> {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        let (ilen, flen) = self.0.count_digits();

        // integer part I
        if ilen > I {
            return Err(DecimalFormatError::IntegerLengthExceeded { ilen, max: I }.into());
        }

        // fractional part F
        if flen > F {
            return Err(DecimalFormatError::FractionalLengthExceeded { flen, max: F }.into());
        }

        Ok(())
    }
}

///
/// Degrees (°)
///

#[newtype(
    primitive = "U16",
    value(item(is = "types::U16")),
    validator(path = "validator::number::Range", args(0, 360))
)]
pub struct Degrees {}

///
/// Percent
///
/// basic percentage as an integer
///

#[newtype(
    primitive = "U8",
    value(item(is = "types::U8")),
    validator(path = "validator::number::Range", args(0, 100))
)]
pub struct Percent {}

///
/// PercentModifier
///

#[newtype(
    primitive = "U16",
    value(item(is = "types::U16")),
    validator(path = "validator::number::Range", args(0, 10_000))
)]
pub struct PercentModifier {}
