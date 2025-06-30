use crate::{core::traits::ValidatorDecimal, design::prelude::*};
use num_traits::NumCast;

///
/// MaxDecimalPlaces
///

#[validator]
pub struct MaxDecimalPlaces {
    pub target: u32,
}

impl MaxDecimalPlaces {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}
impl ValidatorDecimal for MaxDecimalPlaces {
    fn validate(&self, n: &Decimal) -> Result<(), String> {
        if n.scale() <= self.target {
            Ok(())
        } else {
            Err(format!(
                "{n} must not have more than {} decimal place(s)",
                self.target
            ))
        }
    }
}
