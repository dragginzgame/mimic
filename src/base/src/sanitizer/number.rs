use num_traits::NumCast;

///
/// Clamp
///

#[sanitizer]
pub struct Clamp {}

impl Clamp {
    pub fn sanitize<T>(n: &T, min: T, max: T) -> T
    where
        T: Ord + PartialOrd + NumCast + Copy,
    {
        (*n).clamp(min, max)
    }
}
