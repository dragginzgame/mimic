use crate::core::{
    traits::{FieldValue, TypeView, ValidateAuto, ValidateCustom, Visitable},
    value::Value,
};
use candid::CandidType;
use derive_more::{Add, AddAssign, Deref, DerefMut, Display, FromStr, Sub, SubAssign};
use num_traits::{FromPrimitive, NumCast, ToPrimitive};
use rust_decimal::Decimal as WrappedDecimal;
use serde::{Deserialize, Serialize, ser::Error};
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

    #[must_use]
    pub fn count_digits(&self) -> (usize, usize) {
        let str = format!("{}", self.0.abs());
        let parts: Vec<_> = str.split('.').collect();

        let id = parts[0].len();
        let fd = if parts.len() > 1 { parts[1].len() } else { 0 };

        (id, fd)
    }

    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        self.0.checked_rem(*rhs).map(Self)
    }

    #[must_use]
    pub fn from_i128_with_scale(num: i128, scale: u32) -> Self {
        WrappedDecimal::from_i128_with_scale(num, scale).into()
    }
}

impl CandidType for Decimal {
    fn _ty() -> candid::types::Type {
        candid::types::TypeInner::Float64.into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        let v: f64 = self
            .0
            .to_f64()
            .ok_or_else(|| S::Error::custom("Failed to convert Decimal to f64"))?;

        serializer.serialize_float64(v)
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
