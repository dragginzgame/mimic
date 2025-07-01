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

const SCALE: u128 = 1_000_000_000_000_000_000;

///
/// E18s
///
/// Ethereum-style fixed-point: u128 represents value × 1e18.
/// For example, 1.25 = 1_250_000_000_000_000_000.
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
pub struct E18s(pub u128);

impl E18s {
    #[must_use]
    pub fn from_tokens(value: f64) -> Option<Self> {
        if value.is_nan() || value.is_infinite() || value < 0.0 {
            return None;
        }
        Some(Self((value * SCALE as f64).round() as u128))
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
            let mut s = format!("{frac:018}");
            while s.ends_with('0') {
                s.pop();
            }
            s.len()
        };

        (id, fd)
    }
}

impl Display for E18s {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scaled = self.0;
        let whole = scaled / SCALE;
        let frac = scaled % SCALE;

        if frac == 0 {
            write!(f, "{whole}")
        } else {
            let mut frac_str = format!("{frac:018}");
            while frac_str.ends_with('0') {
                frac_str.pop();
            }
            write!(f, "{whole}.{frac_str}")
        }
    }
}

impl FieldSearchable for E18s {
    fn to_searchable_string(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl FieldSortable for E18s {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldValue for E18s {
    fn to_value(&self) -> Value {
        Value::E18s(*self)
    }
}

impl From<u128> for E18s {
    fn from(n: u128) -> Self {
        Self(n)
    }
}

impl Inner for E18s {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        *self
    }

    fn into_inner(self) -> Self::Primitive {
        self
    }
}

impl ValidateAuto for E18s {}

impl ValidateCustom for E18s {}

impl Visitable for E18s {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_and_to_f64_round_trip() {
        let original = 1.234_567_891_234_567;
        let e18s = E18s::from_tokens(original).unwrap();
        let result = e18s.to_tokens();
        let diff = (original - result).abs();
        assert!(diff < 1e-18, "round-trip error too large: {diff}");
    }

    #[test]
    fn test_display_formatting() {
        let e18s = E18s::from_tokens(42.5).unwrap();
        assert_eq!("42.5", e18s.to_string());
    }

    #[test]
    fn test_equality_and_ordering() {
        let a = E18s::from_tokens(10.0).unwrap();
        let b = E18s::from_tokens(20.0).unwrap();
        let c = E18s::from_tokens(10.0).unwrap();

        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, c);
    }

    #[test]
    fn test_count_digits() {
        let e18s = E18s::from_str("123456789123456789123").unwrap(); // 21 digits
        let (int_digits, frac_digits) = e18s.count_digits();
        assert_eq!(int_digits, 3);
        assert_eq!(frac_digits, 18);
    }

    #[test]
    fn test_to_searchable_string() {
        let e18s = E18s::from_str("317").unwrap();
        let search = e18s.to_searchable_string().unwrap();

        assert_eq!(search, "0.000000000000000317");
    }

    #[test]
    fn test_from_u128() {
        let raw = 42 * SCALE;
        let e18s = E18s::from(raw);
        assert_eq!(e18s.to_tokens(), 42.0);
    }

    #[test]
    fn test_default_is_zero() {
        let fixed = E18s::default();
        assert_eq!(fixed.to_tokens(), 0.0);
    }

    #[test]
    fn test_nan_and_infinity_rejection() {
        assert!(E18s::from_tokens(f64::NAN).is_none());
        assert!(E18s::from_tokens(f64::INFINITY).is_none());
        assert!(E18s::from_tokens(f64::NEG_INFINITY).is_none());
        assert!(E18s::from_tokens(-1.0).is_none());
    }
}
