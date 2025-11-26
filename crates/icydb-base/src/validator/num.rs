use crate::{core::traits::Validator, prelude::*};

///
/// Lt
///

#[validator]
pub struct Lt {
    target: Decimal,
}

impl Lt {
    pub fn new<N: NumCast>(target: N) -> Self {
        let target = <Decimal as NumCast>::from(target).unwrap();

        Self { target }
    }
}

impl<N: NumCast + Clone> Validator<N> for Lt {
    fn validate(&self, value: &N) -> Result<(), String> {
        let v = <Decimal as NumCast>::from(value.clone()).unwrap();

        if v < self.target {
            Ok(())
        } else {
            Err(format!("{} must be < {}", v, self.target))
        }
    }
}

///
/// Gt
///

#[validator]
pub struct Gt {
    target: Decimal,
}

impl Gt {
    pub fn new<N: NumCast>(target: N) -> Self {
        let target = <Decimal as NumCast>::from(target).unwrap();

        Self { target }
    }
}

impl<N: NumCast + Clone> Validator<N> for Gt {
    fn validate(&self, value: &N) -> Result<(), String> {
        let v = <Decimal as NumCast>::from(value.clone()).unwrap();

        if v > self.target {
            Ok(())
        } else {
            Err(format!("{} must be > {}", v, self.target))
        }
    }
}

///
/// Lte
///

#[validator]
pub struct Lte {
    target: Decimal,
}

impl Lte {
    pub fn new<N: NumCast>(target: N) -> Self {
        let target = <Decimal as NumCast>::from(target).unwrap();

        Self { target }
    }
}

impl<N: NumCast + Clone> Validator<N> for Lte {
    fn validate(&self, value: &N) -> Result<(), String> {
        let v = <Decimal as NumCast>::from(value.clone()).unwrap();

        if v <= self.target {
            Ok(())
        } else {
            Err(format!("{} must be <= {}", v, self.target))
        }
    }
}

///
/// Gte
///

#[validator]
pub struct Gte {
    target: Decimal,
}

impl Gte {
    pub fn new<N: NumCast>(target: N) -> Self {
        let target = <Decimal as NumCast>::from(target).unwrap();

        Self { target }
    }
}

impl<N: NumCast + Clone> Validator<N> for Gte {
    fn validate(&self, value: &N) -> Result<(), String> {
        let v = <Decimal as NumCast>::from(value.clone()).unwrap();

        if v >= self.target {
            Ok(())
        } else {
            Err(format!("{} must be >= {}", v, self.target))
        }
    }
}

///
/// Equal
///

#[validator]
pub struct Equal {
    target: Decimal,
}

impl Equal {
    pub fn new<N: NumCast>(target: N) -> Self {
        let target = <Decimal as NumCast>::from(target).unwrap();

        Self { target }
    }
}

impl<N: NumCast + Clone> Validator<N> for Equal {
    fn validate(&self, value: &N) -> Result<(), String> {
        let v = <Decimal as NumCast>::from(value.clone()).unwrap();

        if v == self.target {
            Ok(())
        } else {
            Err(format!("{} must be == {}", v, self.target))
        }
    }
}

///
/// NotEqual
///

#[validator]
pub struct NotEqual {
    target: Decimal,
}

impl NotEqual {
    pub fn new<N: NumCast>(target: N) -> Self {
        let target = <Decimal as NumCast>::from(target).unwrap();

        Self { target }
    }
}

impl<N: NumCast + Clone> Validator<N> for NotEqual {
    fn validate(&self, value: &N) -> Result<(), String> {
        let v = <Decimal as NumCast>::from(value.clone()).unwrap();

        if v == self.target {
            Err(format!("{} must be != {}", v, self.target))
        } else {
            Ok(())
        }
    }
}

///
/// Range
///

#[validator]
pub struct Range {
    min: Decimal,
    max: Decimal,
}

impl Range {
    pub fn new<N: NumCast>(min: N, max: N) -> Self {
        let min = <Decimal as NumCast>::from(min).unwrap();
        let max = <Decimal as NumCast>::from(max).unwrap();
        assert!(min <= max, "range requires min <= max");

        Self { min, max }
    }
}

impl<N: NumCast + Clone> Validator<N> for Range {
    fn validate(&self, n: &N) -> Result<(), String> {
        let v = <Decimal as NumCast>::from(n.clone()).unwrap();

        if v < self.min || v > self.max {
            Err(format!(
                "{} must be between {} and {}",
                v, self.min, self.max
            ))
        } else {
            Ok(())
        }
    }
}

///
/// MultipleOf
///

#[validator]
pub struct MultipleOf {
    target: Decimal,
}

impl MultipleOf {
    pub fn new<N: NumCast>(target: N) -> Self {
        let target = <Decimal as NumCast>::from(target).unwrap();
        Self { target }
    }
}

impl<N: NumCast + Clone> Validator<N> for MultipleOf {
    fn validate(&self, n: &N) -> Result<(), String> {
        let v = <Decimal as NumCast>::from(n.clone()).unwrap();

        if (*v % *self.target).is_zero() {
            Ok(())
        } else {
            Err(format!("{v} is not a multiple of {}", self.target))
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

        let result = Lt::new(5.1).validate(&5.0);
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
