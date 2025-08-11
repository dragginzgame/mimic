use crate::core::{
    traits::{FieldValue, TypeView, ValidateAuto, ValidateCustom, Visitable},
    types::Decimal,
    value::Value,
};
use candid::CandidType;
use derive_more::{Add, AddAssign, FromStr, Sub, SubAssign};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

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
pub struct E18s(u128);

impl E18s {
    const SCALE: u128 = 1_000_000_000_000_000_000; // 1e18

    #[must_use]
    pub fn from_decimal(value: Decimal) -> Option<Self> {
        let d = value * Self::SCALE;

        Some(Self(d.to_u128()?))
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

        Some(Self((value * Self::SCALE as f64).round() as u128))
    }

    #[must_use]
    pub const fn get(self) -> u128 {
        self.0
    }

    #[must_use]
    pub fn to_decimal(self) -> Decimal {
        Decimal::from_i128_with_scale(self.0 as i128, 18)
    }

    #[must_use]
    pub fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    #[must_use]
    pub fn count_digits(&self) -> (usize, usize) {
        let whole = self.0 / Self::SCALE;
        let frac = self.0 % Self::SCALE;

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
        write!(f, "{:.18}", self.to_decimal())
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

impl TypeView for E18s {
    type View = u128;

    fn to_view(&self) -> Self::View {
        self.0
    }

    fn from_view(view: Self::View) -> Self {
        Self(view)
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
    fn test_display_formatting() {
        let dec = Decimal::from_str("42.5").unwrap();
        let e18s = E18s::from_decimal(dec).unwrap();

        assert_eq!(e18s.to_string(), "42.5");
    }

    #[test]
    fn test_equality_and_ordering() {
        let a = E18s::from_decimal(Decimal::from_str("10.0").unwrap()).unwrap();
        let b = E18s::from_decimal(Decimal::from_str("20.0").unwrap()).unwrap();
        let c = E18s::from_decimal(Decimal::from_str("10.0").unwrap()).unwrap();

        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, c);
    }

    #[test]
    fn test_count_digits() {
        let e18s =
            E18s::from_decimal(Decimal::from_str("123.456789123456789123").unwrap()).unwrap();
        let (int_digits, frac_digits) = e18s.count_digits();

        assert_eq!(int_digits, 3);
        assert_eq!(frac_digits, 18);
    }

    #[test]
    fn test_from_u128() {
        let raw = 42 * E18s::SCALE;
        let e18s = E18s::from(raw);

        assert_eq!(e18s.to_decimal(), Decimal::from(42));
    }

    #[test]
    fn test_default_is_zero() {
        let fixed = E18s::default();

        assert_eq!(fixed.to_decimal(), Decimal::ZERO);
    }

    #[test]
    fn test_nan_and_infinity_rejection() {
        assert!(E18s::from_f64(f64::NAN).is_none());
        assert!(E18s::from_f64(f64::INFINITY).is_none());
        assert!(E18s::from_f64(f64::NEG_INFINITY).is_none());
    }
}
