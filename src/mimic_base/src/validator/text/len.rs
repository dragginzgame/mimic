use crate::prelude::*;
use num_traits::NumCast;

const MAX_DISPLAY_CHARS: usize = 20;

// truncate_string
fn truncate_string<S: ToString>(s: &S) -> String {
    let string = s.to_string();

    if string.len() > MAX_DISPLAY_CHARS {
        format!("{}...", &string[..MAX_DISPLAY_CHARS])
    } else {
        string
    }
}

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

impl ValidatorString for Equal {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let string = s.as_ref();
        let len = string.len();

        if len == self.target {
            Ok(())
        } else {
            Err(format!(
                "length of '{}' ({}) is not equal to {}",
                truncate_string(&string),
                len,
                self.target
            ))
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

impl ValidatorString for Min {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let string = s.as_ref();
        let len = string.len();

        if len < self.target {
            Err(format!(
                "length of '{}' ({}) is lower than minimum of {}",
                truncate_string(&string),
                len,
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

impl ValidatorString for Max {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let string = s.as_ref();
        let len = string.len();

        if len > self.target {
            Err(format!(
                "length of '{}' ({}) is greater than maximum of {}",
                truncate_string(&string),
                len,
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

impl ValidatorString for Range {
    fn validate<S: AsRef<str>>(&self, s: S) -> Result<(), String> {
        let min = Min::new(self.min);
        min.validate(&s)?;

        let max = Max::new(self.max);
        max.validate(&s)?;

        Ok(())
    }
}
