use crate::{core::traits::Sanitizer, design::prelude::*};

///
/// Clamp
/// Works with integers, floats, and Decimal (any PartialOrd + Clone)
///

#[sanitizer]
pub struct Clamp<T> {
    pub min: T,
    pub max: T,
    pub target: T,
}

impl<T: PartialOrd + Clone> Clamp<T> {
    pub const fn new(min: T, max: T, target: T) -> Self {
        Self { min, max, target }
    }
}

impl<T: PartialOrd + Clone> Sanitizer<T> for Clamp<T> {
    fn sanitize(&self, value: T) -> T {
        if value < self.min {
            self.min.clone()
        } else if value > self.max {
            self.max.clone()
        } else {
            value
        }
    }
}
