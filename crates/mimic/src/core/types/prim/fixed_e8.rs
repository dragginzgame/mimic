use crate::core::{
    traits::{
        FieldSearchable, FieldSortable, FieldValue, Inner, ValidateAuto, ValidateCustom, Visitable,
    },
    value::Value,
};
use candid::CandidType;
use derive_more::{Add, AddAssign, Deref, DerefMut, FromStr, Sub, SubAssign};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

const SCALE: u64 = 100_000_000;

///
/// FixedE8
///
/// Stores numbers as u64 representing value × 1e8
/// For example: 1.25 = 125_000_000
///

#[derive(
    Add,
    AddAssign,
    CandidType,
    Clone,
    Copy,
    Debug,
    Default,
    Deref,
    DerefMut,
    Eq,
    FromStr,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    Sub,
    SubAssign,
)]
pub struct FixedE8(u64);

impl FixedE8 {
    #[must_use]
    pub fn from_tokens(value: f64) -> Option<Self> {
        if value.is_nan() || value.is_infinite() {
            return None;
        }
        Some(Self((value * SCALE as f64).round() as u64))
    }

    #[must_use]
    pub fn to_tokens(self) -> f64 {
        self.0 as f64 / SCALE as f64
    }

    #[must_use]
    pub fn count_digits(&self) -> (usize, usize) {
        let whole = self.0 / SCALE;
        let frac = self.0 % SCALE;

        let id = whole.to_string().len();
        let fd = {
            let mut s = format!("{frac:08}");
            while s.ends_with('0') {
                s.pop();
            }
            s.len()
        };

        (id, fd)
    }
}

impl Display for FixedE8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.8}", self.to_tokens())
    }
}

impl FieldSearchable for FixedE8 {
    fn to_searchable_string(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl FieldSortable for FixedE8 {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldValue for FixedE8 {
    fn to_value(&self) -> Value {
        Value::FixedE8(*self)
    }
}

impl From<u64> for FixedE8 {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl Inner for FixedE8 {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        *self
    }

    fn into_inner(self) -> Self::Primitive {
        self
    }
}

impl ValidateAuto for FixedE8 {}

impl ValidateCustom for FixedE8 {}

impl Visitable for FixedE8 {}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_and_to_f64_round_trip() {
        let original = 1.23456789;
        let fixed = FixedE8::from_tokens(original).unwrap();
        let result = fixed.to_tokens();
        let diff = (original - result).abs();
        assert!(diff < 1e-8, "round-trip error too large: {diff}");
    }

    #[test]
    fn test_display_formatting() {
        let fixed = FixedE8::from_tokens(42.5).unwrap();
        assert_eq!("42.50000000".to_string(), fixed.to_string());
    }

    #[test]
    fn test_equality_and_ordering() {
        let a = FixedE8::from_tokens(10.0).unwrap();
        let b = FixedE8::from_tokens(20.0).unwrap();
        let c = FixedE8::from_tokens(10.0).unwrap();

        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, c);
    }

    #[test]
    fn test_count_digits() {
        let fixed = FixedE8::from_tokens(123.456789).unwrap();
        let (int_digits, frac_digits) = fixed.count_digits();
        assert_eq!(int_digits, 3);
        assert_eq!(frac_digits, 6); // .456789
    }

    #[test]
    fn test_to_searchable_string() {
        let fixed = FixedE8::from_tokens(3.14).unwrap();
        let search = fixed.to_searchable_string().unwrap();
        assert_eq!(search, "3.14000000");
    }

    #[test]
    fn test_from_u64() {
        let fixed = FixedE8::from_tokens(42.0);
        assert_eq!(fixed.unwrap().to_tokens(), 42.0);
    }

    #[test]
    fn test_default_is_zero() {
        let fixed = FixedE8::default();
        assert_eq!(fixed.to_tokens(), 0.0);
    }

    #[test]
    fn test_nan_and_infinity_rejection() {
        assert!(FixedE8::from_tokens(f64::NAN).is_none());
        assert!(FixedE8::from_tokens(f64::INFINITY).is_none());
        assert!(FixedE8::from_tokens(f64::NEG_INFINITY).is_none());
    }
}
