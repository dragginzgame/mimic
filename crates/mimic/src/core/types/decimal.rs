use crate::core::{
    traits::{FieldValue, TypeView, ValidateAuto, ValidateCustom, Visitable},
    value::Value,
};
use candid::CandidType;
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, FromStr, Sub, SubAssign, Sum};
use num_traits::{FromPrimitive, NumCast, ToPrimitive};
use rust_decimal::Decimal as WrappedDecimal;
use serde::{Deserialize, Serialize};
use std::ops::{Div, Mul};

///
/// Decimal
///

#[derive(
    Add,
    AddAssign,
    Clone,
    Copy,
    Debug,
    Default,
    Deref,
    DerefMut,
    Display,
    Eq,
    FromStr,
    PartialEq,
    Sum,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    Sub,
    SubAssign,
)]
pub struct Decimal(WrappedDecimal);

impl Decimal {
    pub const ZERO: Self = Self(WrappedDecimal::ZERO);

    #[must_use]
    pub fn new(num: i64, scale: u32) -> Self {
        Self(WrappedDecimal::new(num, scale))
    }

    ///
    /// WRAPPED FUNCTIONS
    ///

    #[must_use]
    pub fn round_dp(&self, dp: u32) -> Self {
        Self(self.0.round_dp(dp))
    }

    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        self.0.checked_rem(*rhs).map(Self)
    }

    #[must_use]
    pub fn from_i128_with_scale(num: i128, scale: u32) -> Self {
        WrappedDecimal::from_i128_with_scale(num, scale).into()
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        Self(self.0.normalize())
    }
}

impl CandidType for Decimal {
    fn _ty() -> candid::types::Type {
        candid::types::TypeInner::Text.into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        serializer.serialize_text(&self.0.to_string())
    }
}

impl<D: Into<Self>> Div<D> for Decimal {
    type Output = Self;

    fn div(self, d: D) -> Self::Output {
        let rhs: Self = d.into();
        Self(self.0 / rhs.0)
    }
}

impl FieldValue for Decimal {
    fn to_value(&self) -> Value {
        Value::Decimal(*self)
    }
}

impl FromPrimitive for Decimal {
    fn from_i64(n: i64) -> Option<Self> {
        Some(WrappedDecimal::from(n).into())
    }

    fn from_u64(n: u64) -> Option<Self> {
        WrappedDecimal::from_u64(n).map(Self)
    }

    fn from_f32(n: f32) -> Option<Self> {
        WrappedDecimal::from_f32(n).map(Into::into)
    }

    fn from_f64(n: f64) -> Option<Self> {
        WrappedDecimal::from_f64(n).map(Into::into)
    }
}

impl From<WrappedDecimal> for Decimal {
    fn from(d: WrappedDecimal) -> Self {
        Self(d)
    }
}

// lossy f32 done on purpose as these ORM floats aren't designed for NaN etc.
impl From<f32> for Decimal {
    fn from(n: f32) -> Self {
        if n.is_finite() {
            WrappedDecimal::from_f32(n).unwrap_or(Self::ZERO.0).into()
        } else {
            Self::ZERO
        }
    }
}

impl From<f64> for Decimal {
    fn from(n: f64) -> Self {
        if n.is_finite() {
            WrappedDecimal::from_f64(n).unwrap_or(Self::ZERO.0).into()
        } else {
            Self::ZERO
        }
    }
}

macro_rules! impl_decimal_from_int {
    ( $( $type:ty ),* ) => {
        $(
            impl From<$type> for Decimal {
                fn from(n: $type) -> Self {
                    Self(rust_decimal::Decimal::from(n))
                }
            }
        )*
    };
}

impl_decimal_from_int!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl<D: Into<Self>> Mul<D> for Decimal {
    type Output = Self;

    fn mul(self, d: D) -> Self::Output {
        let rhs: Self = d.into();
        Self(self.0 * rhs.0)
    }
}

impl NumCast for Decimal {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        WrappedDecimal::from_f64(n.to_f64()?).map(Decimal)
    }
}

impl ToPrimitive for Decimal {
    fn to_i32(&self) -> Option<i32> {
        self.0.to_i32()
    }

    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }

    fn to_u128(&self) -> Option<u128> {
        self.0.to_u128()
    }

    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl TypeView for Decimal {
    type View = Self;

    fn to_view(&self) -> Self::View {
        *self
    }

    fn from_view(view: Self::View) -> Self {
        Self(*view)
    }
}

impl ValidateAuto for Decimal {}

impl ValidateCustom for Decimal {}

impl Visitable for Decimal {}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use candid::{decode_one, encode_one};
    use std::str::FromStr;

    #[test]
    fn decimal_candid_roundtrip() {
        let cases = [
            "0",
            "1",
            "-1",
            "42.5",
            "1234567890.123456789",
            "0.00000001",
            "1000000000000000000000000.000000000000000000000001",
        ];

        for s in cases {
            let d1 = Decimal::from_str(s).expect("parse decimal");

            // encode via Candid (should encode as text)
            let bytes = encode_one(d1).expect("candid encode");

            // decode back to Decimal
            let d2: Decimal = decode_one(&bytes).expect("candid decode to Decimal");
            assert_eq!(d2, d1, "roundtrip mismatch for {s}");

            // also ensure the on-wire representation is text by decoding as String
            let wire_str: String = decode_one(&bytes).expect("candid decode to String");
            assert_eq!(wire_str, d1.0.to_string(), "wire text mismatch for {s}");
        }
    }
}
