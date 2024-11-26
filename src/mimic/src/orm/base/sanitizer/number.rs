use crate::orm::{prelude::*, traits::NumCast};
use types::Decimal;

///
/// Clamp
///

#[sanitizer]
pub struct Clamp {
    pub min: Decimal,
    pub max: Decimal,
}

impl Clamp {
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

impl Sanitizer for Clamp {
    fn sanitize_number<N>(&self, n: &N) -> Result<N, String>
    where
        N: Copy + Display + NumCast,
    {
        if let Some(n_cast) = <Decimal as NumCast>::from(*n) {
            let clamped = n_cast.clamp(self.min, self.max);
            if let Some(result) = N::from(clamped) {
                Ok(result)
            } else {
                Err(format!(
                    "Failed to cast clamped value {clamped} back to the target type"
                ))
            }
        } else {
            Err(format!("Failed to cast input value {n} to Decimal"))
        }
    }
}
