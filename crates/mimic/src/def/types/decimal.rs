use crate::{
    def::traits::{FieldOrderable, Inner, ValidateAuto, Visitable},
    prelude::*,
};
use candid::CandidType;
use derive_more::{Add, AddAssign, Deref, DerefMut, FromStr, Sub, SubAssign};
use num_traits::{FromPrimitive, NumCast, ToPrimitive};
use rust_decimal::Decimal as WrappedDecimal;
use serde::{Deserialize, Serialize, ser::Error};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

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
    #[must_use]
    pub fn new(num: i64, scale: u32) -> Self {
        Self(WrappedDecimal::new(num, scale))
    }

    // count_digits
    #[must_use]
    pub fn count_digits(&self) -> (usize, usize) {
        let str = format!("{}", self.0.abs());
        let parts: Vec<_> = str.split('.').collect();

        let id = parts[0].len();
        let fd = if parts.len() > 1 { parts[1].len() } else { 0 };

        (id, fd)
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

impl Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FieldOrderable for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl From<i32> for Decimal {
    fn from(n: i32) -> Self {
        Self(n.into())
    }
}

impl From<i64> for Decimal {
    fn from(n: i64) -> Self {
        Self(n.into())
    }
}

impl From<isize> for Decimal {
    fn from(n: isize) -> Self {
        Self(n.into())
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<f32> for Decimal {
    fn from(n: f32) -> Self {
        Self(WrappedDecimal::from_f32(n).unwrap())
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<f64> for Decimal {
    fn from(n: f64) -> Self {
        Self(WrappedDecimal::from_f64(n).unwrap())
    }
}

impl From<WrappedDecimal> for Decimal {
    fn from(d: WrappedDecimal) -> Self {
        Self(d)
    }
}

impl From<Decimal> for WrappedDecimal {
    fn from(d: Decimal) -> Self {
        *d
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

impl Inner for Decimal {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        *self
    }

    fn into_inner(self) -> Self::Primitive {
        self
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

    fn to_f32(&self) -> Option<f32> {
        self.0.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl ValidateAuto for Decimal {}

impl ValidateCustom for Decimal {}

impl Visitable for Decimal {}
