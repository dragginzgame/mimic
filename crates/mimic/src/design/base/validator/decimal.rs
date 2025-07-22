use crate::{core::traits::Validator, design::prelude::*};
use num_traits::NumCast;

///
/// MaxDecimalPlaces
///

#[validator(fields(field(name = "target", value(item(prim = "Nat32")))))]
pub struct MaxDecimalPlaces {}

impl MaxDecimalPlaces {
    pub fn new<N: NumCast>(target: N) -> Self {
        Self {
            target: NumCast::from(target).unwrap(),
        }
    }
}
impl Validator<Decimal> for MaxDecimalPlaces {
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
