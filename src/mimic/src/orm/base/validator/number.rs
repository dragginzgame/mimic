use crate::orm::prelude::*;
use num_traits::{CheckedRem, NumCast, Zero};

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("failed to convert value to the validator's native type"))]
    ConversionFailed,

    #[snafu(display("{n} must be less than {than}"))]
    NotLt { n: String, than: String },

    #[snafu(display("{n} must be greater than {than}"))]
    NotGt { n: String, than: String },

    #[snafu(display("{n} must be less than or equal to {than}"))]
    NotLtoe { n: String, than: String },

    #[snafu(display("{n} must be greater than or equal to {than}"))]
    NotGtoe { n: String, than: String },

    #[snafu(display("{n} must be equal to {to}"))]
    NotEqualTo { n: String, to: String },

    #[snafu(display("{n} cannot be equal to {to}"))]
    EqualTo { n: String, to: String },

    #[snafu(display("{n} is not within the range of {min} to {max}"))]
    NotInRange { n: String, min: String, max: String },

    #[snafu(display("{n} is not a multiple of {of}"))]
    NotMultipleOf { n: String, of: String },

    #[snafu(display("{n} is not found within the specified list of values"))]
    NotInArray { n: String },
}

///
/// Lt
///

#[validator]
pub struct Lt {}

impl Lt {
    pub fn validate<T, U>(n: &T, than: U) -> Result<(), Error>
    where
        T: Copy + Display + PartialOrd,
        U: Copy + Display + TryInto<T>,
    {
        let than: T = than.try_into().map_err(|_| Error::ConversionFailed)?;

        if *n < than {
            Ok(())
        } else {
            Err(Error::NotLt {
                n: n.to_string(),
                than: than.to_string(),
            })
        }
    }
}

///
/// Gt
///

#[validator]
pub struct Gt {}

impl Gt {
    pub fn validate<T, U>(n: &T, than: U) -> Result<(), Error>
    where
        T: Copy + Display + PartialOrd,
        U: Copy + Display + TryInto<T>,
    {
        let than: T = than.try_into().map_err(|_| Error::ConversionFailed)?;

        if *n > than {
            Ok(())
        } else {
            Err(Error::NotGt {
                n: n.to_string(),
                than: than.to_string(),
            })
        }
    }
}

///
/// Ltoe
///

#[validator]
pub struct Ltoe {}

impl Ltoe {
    pub fn validate<T, U>(n: &T, than: U) -> Result<(), Error>
    where
        T: Copy + Display + PartialOrd,
        U: Copy + Display + TryInto<T>,
    {
        let than: T = than.try_into().map_err(|_| Error::ConversionFailed)?;

        if *n <= than {
            Ok(())
        } else {
            Err(Error::NotLtoe {
                n: n.to_string(),
                than: than.to_string(),
            })
        }
    }
}

///
/// Gtoe
///

#[validator]
pub struct Gtoe {}

impl Gtoe {
    pub fn validate<T, U>(n: &T, than: U) -> Result<(), Error>
    where
        T: Copy + Display + PartialOrd,
        U: Copy + Display + TryInto<T>,
    {
        let than: T = than.try_into().map_err(|_| Error::ConversionFailed)?;

        if *n >= than {
            Ok(())
        } else {
            Err(Error::NotGtoe {
                n: n.to_string(),
                than: than.to_string(),
            })
        }
    }
}

///
/// Equal
///

#[validator]
pub struct Equal {}

impl Equal {
    pub fn validate<T, U>(n: &T, to: U) -> Result<(), Error>
    where
        T: Copy + Display + PartialEq,
        U: Copy + Display + TryInto<T>,
    {
        let to: T = to.try_into().map_err(|_| Error::ConversionFailed)?;

        if *n == to {
            Ok(())
        } else {
            Err(Error::NotEqualTo {
                n: n.to_string(),
                to: to.to_string(),
            })
        }
    }
}

///
/// NotEqual
///

#[validator]
pub struct NotEqual {}

impl NotEqual {
    pub fn validate<T, U>(n: &T, to: U) -> Result<(), Error>
    where
        T: Copy + Display + PartialEq,
        U: Copy + Display + TryInto<T>,
    {
        let to: T = to.try_into().map_err(|_| Error::ConversionFailed)?;

        if *n == to {
            Err(Error::EqualTo {
                n: n.to_string(),
                to: to.to_string(),
            })
        } else {
            Ok(())
        }
    }
}

///
/// Range
///

#[validator]
pub struct Range {}

impl Range {
    pub fn validate<T, U>(n: &T, min: U, max: U) -> Result<(), Error>
    where
        T: Copy + Display + PartialOrd,
        U: Copy + Display + TryInto<T>,
    {
        let min: T = min.try_into().map_err(|_| Error::ConversionFailed)?;
        let max: T = max.try_into().map_err(|_| Error::ConversionFailed)?;

        if *n >= min && *n <= max {
            Ok(())
        } else {
            Err(Error::NotInRange {
                n: n.to_string(),
                min: min.to_string(),
                max: max.to_string(),
            })
        }
    }
}

///
/// MultipleOf
///

#[validator]
pub struct MultipleOf {}

impl MultipleOf {
    pub fn validate<T, U>(n: &T, of: U) -> Result<(), Error>
    where
        T: Display + NumCast + Copy,
        U: Display + NumCast + PartialOrd + CheckedRem + Zero,
    {
        let n_cast = U::from(*n).ok_or(Error::ConversionFailed)?;
        let zero = U::zero();

        if n_cast.checked_rem(&of) == Some(zero) {
            Ok(())
        } else {
            Err(Error::NotMultipleOf {
                n: n.to_string(),
                of: of.to_string(),
            })
        }
    }
}

///
/// InArray
///

#[validator]
pub struct InArray {}

impl InArray {
    pub fn validate<T, U>(n: &T, valid_values: &[U]) -> Result<(), Error>
    where
        T: Display + NumCast + Copy,
        U: Display + NumCast + PartialOrd,
    {
        let n_cast = U::from(*n).ok_or(Error::ConversionFailed)?;

        if valid_values.iter().any(|value| n_cast == *value) {
            Ok(())
        } else {
            Err(Error::NotInArray { n: n.to_string() })
        }
    }
}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lt_validator_success() {
        let result = Lt::validate(&5, 10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lt_validator_failure() {
        let result = Lt::validate(&10, 5);
        assert!(matches!(result, Err(Error::NotLt { .. })));
    }

    #[test]
    fn test_gt_validator_success() {
        let result = Gt::validate(&10, 5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gt_validator_failure() {
        let result = Gt::validate(&5, 10);
        assert!(matches!(result, Err(Error::NotGt { .. })));
    }

    #[test]
    fn test_equal_validator_success() {
        let result = Equal::validate(&5, 5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_equal_validator_failure() {
        let result = Equal::validate(&5, 10);
        assert!(matches!(result, Err(Error::NotEqualTo { .. })));
    }
}
