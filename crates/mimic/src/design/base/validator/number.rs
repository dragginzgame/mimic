use crate::{core::traits::ValidatorNumber, design::prelude::*};
use num_traits::NumCast;

///
/// Helper Functions
///

fn cast_to_decimal<N: Copy + NumCast>(n: &N) -> Result<Decimal, String> {
    NumCast::from(*n).ok_or_else(|| "failed to cast value to decimal".to_string())
}

///
/// Lt
///

#[validator(fields(field(name = "target", value(item(prim = "Decimal")))))]
pub struct Lt {}

impl Lt {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl ValidatorNumber for Lt {
    fn validate<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + NumCast,
    {
        let n_cast = cast_to_decimal(n)?;

        if n_cast < self.target {
            Ok(())
        } else {
            Err(format!("{n_cast} must be less than {}", self.target))
        }
    }
}

///
/// Gt
///

#[validator(fields(field(name = "target", value(item(prim = "Decimal")))))]
pub struct Gt {}

impl Gt {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl ValidatorNumber for Gt {
    fn validate<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + NumCast,
    {
        let n_cast = cast_to_decimal(n)?;

        if n_cast > self.target {
            Ok(())
        } else {
            Err(format!("{n_cast} must be greater than {}", self.target))
        }
    }
}

///
/// Lte
///

#[validator(fields(field(name = "target", value(item(prim = "Decimal")))))]
pub struct Lte {}

impl Lte {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl ValidatorNumber for Lte {
    fn validate<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + NumCast,
    {
        let n_cast = cast_to_decimal(n)?;

        if n_cast <= self.target {
            Ok(())
        } else {
            Err(format!(
                "{n_cast} must be less than or equal to {}",
                self.target
            ))
        }
    }
}

///
/// Gtoe
///

#[validator(fields(field(name = "target", value(item(prim = "Decimal")))))]
pub struct Gtoe {}

impl Gtoe {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl ValidatorNumber for Gtoe {
    fn validate<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + NumCast,
    {
        let n_cast = cast_to_decimal(n)?;

        if n_cast >= self.target {
            Ok(())
        } else {
            Err(format!(
                "{n_cast} must be greater than or equal to {}",
                self.target
            ))
        }
    }
}

///
/// Equal
///

#[validator(fields(field(name = "target", value(item(prim = "Decimal")))))]
pub struct Equal {}

impl Equal {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl ValidatorNumber for Equal {
    fn validate<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + NumCast,
    {
        let n_cast = cast_to_decimal(n)?;

        if n_cast == self.target {
            Ok(())
        } else {
            Err(format!("{n_cast} must be equal to {}", self.target))
        }
    }
}

///
/// NotEqual
///

#[validator(fields(field(name = "target", value(item(prim = "Decimal")))))]
pub struct NotEqual {}

impl NotEqual {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}
impl ValidatorNumber for NotEqual {
    fn validate<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + NumCast,
    {
        let n_cast = cast_to_decimal(n)?;

        if n_cast == self.target {
            Err(format!("{n_cast} must not be equal to {}", self.target))
        } else {
            Ok(())
        }
    }
}

///
/// Range
///

#[validator(fields(
    field(name = "min", value(item(prim = "Decimal"))),
    field(name = "max", value(item(prim = "Decimal"))),
))]
pub struct Range {}

impl Range {
    pub fn new<N>(min: N, max: N) -> Self
    where
        N: NumCast,
    {
        Self {
            min: NumCast::from(min).unwrap(),
            max: NumCast::from(max).unwrap(),
        }
    }
}

impl ValidatorNumber for Range {
    fn validate<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + NumCast,
    {
        let n_cast = cast_to_decimal(n)?;

        if n_cast >= self.min && n_cast <= self.max {
            Ok(())
        } else {
            Err(format!(
                "{n_cast} must be in the range {} to {}",
                self.min, self.max
            ))
        }
    }
}

///
/// MultipleOf
///

#[validator(fields(field(name = "target", value(item(prim = "Decimal")))))]
pub struct MultipleOf {}

impl MultipleOf {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl ValidatorNumber for MultipleOf {
    fn validate<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + NumCast,
    {
        let n_cast = cast_to_decimal(n)?;

        if n_cast.checked_rem(self.target) == Some(Decimal::ZERO) {
            Ok(())
        } else {
            Err(format!("{n_cast} is not a multiple of {}", self.target))
        }
    }
}

///
/// InArray
///

#[validator(fields(field(name = "values", value(many, item(prim = "Int32")))))]
pub struct InArray {}

impl InArray {
    #[must_use]
    pub fn new(values: &[i32]) -> Self {
        Self {
            values: values.to_vec(),
        }
    }
}

impl ValidatorNumber for InArray {
    fn validate<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + NumCast,
    {
        if let Some(n_cast) = <i32 as NumCast>::from(*n) {
            if self.values.contains(&n_cast) {
                Ok(())
            } else {
                Err(format!(
                    "{n_cast} is not in the allowed values: {:?}",
                    self.values
                ))
            }
        } else {
            Err("failed cast to i32".to_string())
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
        let result = Lt::new(10).validate(&5);
        assert!(result.is_ok());

        let result = Lt::new(5.1).validate(&5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lt_validator_failure() {
        let result = Lt::new(5).validate(&10);
        assert!(result.is_err());
    }

    #[test]
    fn test_gt_validator_success() {
        let result = Gt::new(5).validate(&10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gt_validator_failure() {
        let result = Gt::new(10).validate(&5);
        assert!(result.is_err());
    }

    #[test]
    fn test_equal_validator_success() {
        let result = Equal::new(5).validate(&5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_equal_validator_failure() {
        let result = Equal::new(5).validate(&10);
        assert!(result.is_err());
    }
}
