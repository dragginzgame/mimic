use crate::{core::traits::ValidatorNumber, design::prelude::*};
use num_traits::{NumCast, Zero};

///
/// Helper Functions
///

fn cast_to_f64<N: Copy + NumCast>(n: &N) -> Result<f64, String> {
    NumCast::from(*n).ok_or_else(|| "failed to cast value to f64".to_string())
}

///
/// Lt
///

#[validator]
pub struct Lt {
    pub target: f64,
}

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
        let n_cast = cast_to_f64(n)?;

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

#[validator]
pub struct Gt {
    pub target: f64,
}

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
        let n_cast = cast_to_f64(n)?;

        if n_cast > self.target {
            Ok(())
        } else {
            Err(format!("{n_cast} must be greater than {}", self.target))
        }
    }
}

///
/// Ltoe
///

#[validator]
pub struct Ltoe {
    pub target: f64,
}

impl Ltoe {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl ValidatorNumber for Ltoe {
    fn validate<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + NumCast,
    {
        let n_cast = cast_to_f64(n)?;

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

#[validator]
pub struct Gtoe {
    pub target: f64,
}

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
        let n_cast = cast_to_f64(n)?;

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

#[validator]
pub struct Equal {
    pub target: f64,
}

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
        let n_cast = cast_to_f64(n)?;

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

#[validator]
pub struct NotEqual {
    pub target: f64,
}

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
        let n_cast = cast_to_f64(n)?;

        if n_cast != self.target {
            Ok(())
        } else {
            Err(format!("{n_cast} must not be equal to {}", self.target))
        }
    }
}

///
/// Range
///

#[validator]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

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
        let n_cast = cast_to_f64(n)?;

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

#[validator]
pub struct MultipleOf {
    pub target: i32,
}

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
        // cast
        let n_cast: i32 = match NumCast::from(*n) {
            Some(val) => val,
            None => return Err("failed to cast value to i32".to_string()),
        };

        // check
        if n_cast.checked_rem(self.target) == Some(i32::zero()) {
            Ok(())
        } else {
            Err(format!("{n_cast} is not a multiple of {}", self.target))
        }
    }
}

///
/// InArray
///

#[validator]
pub struct InArray {
    values: Vec<i32>,
}

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
