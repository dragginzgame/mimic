use crate::orm::prelude::*;
use num_traits::{NumCast, Zero};

///
/// Lt
///

#[validator]
pub struct Lt {
    pub target: i32,
}

impl Lt {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl Validator for Lt {
    fn validate_number<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + Display + NumCast,
    {
        if let Some(n_cast) = <i32 as NumCast>::from(*n) {
            if n_cast < self.target {
                Ok(())
            } else {
                Err(format!("{n_cast} must be less than {}", self.target))
            }
        } else {
            Err(format!("failed to convert {n} to i32"))
        }
    }
}

///
/// Gt
///

#[validator]
pub struct Gt {
    pub target: i32,
}

impl Gt {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl Validator for Gt {
    fn validate_number<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + Display + NumCast,
    {
        if let Some(n_cast) = <i32 as NumCast>::from(*n) {
            if n_cast > self.target {
                Ok(())
            } else {
                Err(format!("{n_cast} must be greater than {}", self.target))
            }
        } else {
            Err(format!("failed to convert {n} to i32"))
        }
    }
}

///
/// Ltoe
///

#[validator]
pub struct Ltoe {
    pub target: i32,
}

impl Ltoe {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl Validator for Ltoe {
    fn validate_number<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + Display + NumCast,
    {
        if let Some(n_cast) = <i32 as NumCast>::from(*n) {
            if n_cast <= self.target {
                Ok(())
            } else {
                Err(format!(
                    "{n_cast} must be less than or equal to {}",
                    self.target
                ))
            }
        } else {
            Err(format!("failed to convert {n} to i32"))
        }
    }
}

///
/// Gtoe
///

#[validator]
pub struct Gtoe {
    pub target: i32,
}

impl Gtoe {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl Validator for Gtoe {
    fn validate_number<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + Display + NumCast,
    {
        if let Some(n_cast) = <i32 as NumCast>::from(*n) {
            if n_cast >= self.target {
                Ok(())
            } else {
                Err(format!(
                    "{n_cast} must be greater than or equal to {}",
                    self.target
                ))
            }
        } else {
            Err(format!("failed to convert {n} to i32"))
        }
    }
}

///
/// Equal
///

#[validator]
pub struct Equal {
    pub target: i32,
}

impl Equal {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl Validator for Equal {
    fn validate_number<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + Display + NumCast,
    {
        if let Some(n_cast) = <i32 as NumCast>::from(*n) {
            if n_cast == self.target {
                Ok(())
            } else {
                Err(format!("{n_cast} must be equal to {}", self.target))
            }
        } else {
            Err(format!("failed to convert {n} to i32"))
        }
    }
}

///
/// NotEqual
///

#[validator]
pub struct NotEqual {
    pub target: i32,
}

impl NotEqual {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}
impl Validator for NotEqual {
    fn validate_number<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + Display + NumCast,
    {
        if let Some(n_cast) = <i32 as NumCast>::from(*n) {
            if n_cast == self.target {
                Ok(())
            } else {
                Err(format!("{n_cast} must not be equal to {}", self.target))
            }
        } else {
            Err(format!("failed to convert {n} to i32"))
        }
    }
}

///
/// Range
///

#[validator]
pub struct Range {
    pub min: i32,
    pub max: i32,
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

impl Validator for Range {
    fn validate_number<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + Display + NumCast,
    {
        if let Some(n_cast) = <i32 as NumCast>::from(*n) {
            if n_cast >= self.min && n_cast <= self.max {
                Ok(())
            } else {
                Err(format!(
                    "{n_cast} must be in the range {} to {}",
                    self.min, self.max
                ))
            }
        } else {
            Err(format!("failed to convert {n} to i32"))
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

impl Validator for MultipleOf {
    fn validate_number<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + Display + NumCast,
    {
        if let Some(n_cast) = <i32 as NumCast>::from(*n) {
            let zero = i32::zero();

            if n_cast.checked_rem(self.target) == Some(zero) {
                Ok(())
            } else {
                Err(format!("{n_cast} is not a multiple of {}", self.target))
            }
        } else {
            Err(format!("failed to convert {n} to i32"))
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

impl Validator for InArray {
    fn validate_number<N>(&self, n: &N) -> Result<(), String>
    where
        N: Copy + Display + NumCast,
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
            Err(format!("Failed to convert {n} to i32"))
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
        let result = Lt::new(10).validate_number(&5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lt_validator_failure() {
        let result = Lt::new(5).validate_number(&10);
        assert!(result.is_err());
    }

    #[test]
    fn test_gt_validator_success() {
        let result = Gt::new(5).validate_number(&10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gt_validator_failure() {
        let result = Gt::new(10).validate_number(&5);
        assert!(result.is_err());
    }

    #[test]
    fn test_equal_validator_success() {
        let result = Equal::new(5).validate_number(&5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_equal_validator_failure() {
        let result = Equal::new(5).validate_number(&10);
        assert!(result.is_err());
    }
}
