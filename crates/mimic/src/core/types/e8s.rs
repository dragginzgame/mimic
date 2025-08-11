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
/// E8s
///
/// Stores numbers as u64 representing value × 1e8
/// For example: 1.25 = 125_000_000
///

#[repr(transparent)]
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
pub struct E8s(u64);

impl E8s {
    const DECIMALS: u32 = 8;
    const SCALE: u64 = 100_000_000; // 10^8

    ///
    /// CONSTRUCTORS
    ///

    /// Construct from **atomics** (raw scaled integer). No scaling applied.
    #[must_use]
    pub fn from_atomic(raw: u64) -> Self {
        Self(raw)
    }

    /// Construct from **whole units** (tokens). Scales by 1e8.
    #[must_use]
    pub fn from_units(units: u64) -> Self {
        Self(units.saturating_mul(Self::SCALE))
    }

    /// Exact decimal → fixed-point, fails if more than 8 fractional digits.
    #[must_use]
    pub fn try_from_decimal_exact(d: Decimal) -> Option<Self> {
        // multiply and require integer result (no leftover fractional part)
        let scaled = d * Decimal::from(Self::SCALE);

        // require exact integer: normalized equality with its 0dp rounding
        if scaled == scaled.round_dp(0) {
            scaled.to_u64().map(Self)
        } else {
            None
        }
    }

    /// Decimal → fixed-point with rounding to 8dp.
    #[must_use]
    pub fn from_decimal_round(d: Decimal) -> Option<Self> {
        let scaled = (d * Decimal::from(Self::SCALE)).round_dp(0);

        scaled.to_u64().map(E8s)
    }

    ///
    /// METHODS
    ///

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_sign_loss)]
    #[doc = "⚠️ Use only for non-critical float conversions. Prefer try_from_decimal."]
    pub fn from_f64(value: f64) -> Option<Self> {
        if value.is_nan() || value.is_infinite() {
            return None;
        }

        Some(Self((value * Self::SCALE as f64).round() as u64))
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    #[must_use]
    pub fn to_decimal(self) -> Decimal {
        Decimal::from_i128_with_scale(self.0 as i128, Self::DECIMALS).normalize()
    }
}

impl Display for E8s {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_decimal().fmt(f)
    }
}

impl FieldValue for E8s {
    fn to_value(&self) -> Value {
        Value::E8s(*self)
    }
}

impl From<E8s> for Decimal {
    fn from(v: E8s) -> Self {
        // mantissa = raw atomic units, scale = 8
        Self::new(v.get() as i64, 8)
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
        let fixed = E8s::try_from_decimal_exact(dec).unwrap();

        assert_eq!(fixed.to_string(), "42.5");
    }

    #[test]
    fn e8s_units_and_display() {
        let one_unit = E8s::from_units(1);

        assert_eq!(one_unit.get(), E8s::SCALE);
        assert_eq!(one_unit.to_string(), "1"); // because normalize() trims zeros
    }

    #[test]
    fn e8s_raw_and_decimal() {
        let one_atomic = E8s::from(1u64); // raw

        assert_eq!(one_atomic.to_string(), "0.00000001");
    }

    #[test]
    fn e8s_decimal_exact() {
        let x = E8s::try_from_decimal_exact(Decimal::from_str("42.5").unwrap()).unwrap();

        assert_eq!(x.to_string(), "42.5");
        assert_eq!(x.get(), 4_250_000_000);
    }

    #[test]
    fn e8s_decimal_round() {
        // 8dp rounds:
        let x = E8s::from_decimal_round(Decimal::from_str("0.0000000049").unwrap()).unwrap();
        assert_eq!(x.get(), 0); // rounds down
        let y = E8s::from_decimal_round(Decimal::from_str("0.0000000051").unwrap()).unwrap();
        assert_eq!(y.get(), 1); // rounds up
    }

    #[test]
    fn test_default_is_zero() {
        let fixed = E8s::default();
        assert_eq!(fixed.to_decimal(), Decimal::ZERO);
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
