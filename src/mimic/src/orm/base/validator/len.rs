use crate::orm::prelude::*;
use num_traits::NumCast;

///
/// Equal
///

#[validator]
pub struct Equal {
    target: usize,
}

impl Equal {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl Validator for Equal {
    fn validate_string<S: ToString>(&self, s: &S) -> Result<(), String> {
        let len = s.to_string().len();

        if len == self.target {
            Ok(())
        } else {
            Err(format!("length of {len} is not equal to {}", self.target))
        }
    }
}

///
/// Min
///

#[validator]
pub struct Min {
    target: usize,
}

impl Min {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl Validator for Min {
    fn validate_string<S: ToString>(&self, s: &S) -> Result<(), String> {
        let len = s.to_string().len();

        if len < self.target {
            Err(format!(
                "length of {len} is lower than minimum of {}",
                self.target
            ))
        } else {
            Ok(())
        }
    }
}

///
/// Max
///

#[validator]
pub struct Max {
    target: usize,
}

impl Max {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}

impl Validator for Max {
    fn validate_string<S: ToString>(&self, s: &S) -> Result<(), String> {
        let len = s.to_string().len();

        if len > self.target {
            Err(format!(
                "length of {len} is lower than minimum of {}",
                self.target
            ))
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
    min: usize,
    max: usize,
}

impl Range {
    pub fn new<N: NumCast>(min: N, max: N) -> Self {
        Self {
            min: NumCast::from(min).unwrap(),
            max: NumCast::from(max).unwrap(),
        }
    }
}

impl Validator for Range {
    fn validate_string<S: ToString>(&self, s: &S) -> Result<(), String> {
        let len = s.to_string().len();

        if len < self.min {
            Err(format!(
                "length of {len} is lower than the minimum of {}",
                self.min
            ))
        } else if len > self.max {
            Err(format!(
                "length of {len} exceeds the maximum of {}",
                self.max
            ))
        } else {
            Ok(())
        }
    }
}
