use crate::core::{
    traits::{FieldValue, TypeView, ValidateAuto, ValidateCustom, Visitable},
    types::Decimal,
    value::Value,
};
use candid::CandidType;
use derive_more::{Add, AddAssign, Deref, DerefMut, FromStr, Sub, SubAssign};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

const SCALE: u64 = 100_000_000;

///
/// E8s
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
pub struct E8s(u64);

impl E8s {
    #[must_use]
    pub fn from_decimal(value: Decimal) -> Option<Self> {
        let d = value * SCALE;

        Some(Self(d.to_u64()?))
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_sign_loss)]
    #[doc = "⚠️ Use only for non-critical float conversions. Prefer from_decimal."]
    pub fn from_f64(value: f64) -> Option<Self> {
        if value.is_nan() || value.is_infinite() {
            return None;
        }

        Some(Self((value * SCALE as f64).round() as u64))
    }

    #[must_use]
    pub fn to_tokens(self) -> Decimal {
        Decimal::from(self.0) / Decimal::from(SCALE)
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

impl Display for E8s {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.8}", self.to_tokens())
    }
}

impl FieldValue for E8s {
    fn to_value(&self) -> Value {
        Value::E8s(*self)
    }
}

impl From<u64> for E8s {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl TypeView for E8s {
    type View = u64;

    fn to_view(&self) -> Self::View {
        self.0
    }

    fn from_view(view: Self::View) -> Self {
        Self(view)
    }
}

impl ValidateAuto for E8s {}

impl ValidateCustom for E8s {}

impl Visitable for E8s {}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_display_formatting() {
        let dec = Decimal::from_str("42.5").unwrap();
        let fixed = E8s::from_decimal(dec).unwrap();
        assert_eq!(fixed.to_string(), "42.50000000");
    }

    #[test]
    fn test_equality_and_ordering() {
        let a = E8s::from_decimal(Decimal::from_str("10.0").unwrap()).unwrap();
        let b = E8s::from_decimal(Decimal::from_str("20.0").unwrap()).unwrap();
        let c = E8s::from_decimal(Decimal::from_str("10.0").unwrap()).unwrap();

        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, c);
    }

    #[test]
    fn test_count_digits() {
        let dec = Decimal::from_str("123.456789").unwrap();
        let fixed = E8s::from_decimal(dec).unwrap();
        let (int_digits, frac_digits) = fixed.count_digits();
        assert_eq!(int_digits, 3);
        assert_eq!(frac_digits, 6); // .456789
    }

    #[test]
    fn test_from_u64() {
        let fixed = E8s::from_decimal(Decimal::from(42)).unwrap();
        assert_eq!(fixed.to_tokens(), Decimal::from(42));
    }

    #[test]
    fn test_default_is_zero() {
        let fixed = E8s::default();
        assert_eq!(fixed.to_tokens(), Decimal::ZERO);
    }

    #[test]
    fn test_nan_and_infinity_rejection_from_f64() {
        assert!(E8s::from_f64(f64::NAN).is_none());
        assert!(E8s::from_f64(f64::INFINITY).is_none());
        assert!(E8s::from_f64(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_from_f64_accuracy_and_rounding() {
        let val = 0.000_000_004_9_f64;
        let e = E8s::from_f64(val).unwrap();
        assert_eq!(e.0, 0);

        let val = 0.000_000_005_1_f64;
        let e = E8s::from_f64(val).unwrap();
        assert_eq!(e.0, 1);
    }
}
