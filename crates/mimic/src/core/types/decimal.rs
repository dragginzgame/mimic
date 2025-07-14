use crate::core::{
    traits::{
        FieldSearchable, FieldSortable, FieldValue, Inner, TypeView, ValidateAuto, ValidateCustom,
        Visitable,
    },
    value::Value,
};
use candid::CandidType;
use derive_more::{Add, AddAssign, Deref, DerefMut, FromStr, Sub, SubAssign};
use num_traits::{FromPrimitive, NumCast, ToPrimitive};
use rust_decimal::Decimal as WrappedDecimal;
use serde::{Deserialize, Serialize, ser::Error};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
    ops::{Div, Mul},
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

impl<D: Into<Self>> Div<D> for Decimal {
    type Output = Self;

    fn div(self, d: D) -> Self::Output {
        let rhs: Self = d.into();
        Self(self.0 / rhs.0)
    }
}

impl FieldSortable for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl FieldSearchable for Decimal {
    fn to_searchable_string(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl FieldValue for Decimal {
    fn to_value(&self) -> Value {
        Value::Decimal(*self)
    }
}

impl<D: Into<WrappedDecimal>> From<D> for Decimal {
    fn from(d: D) -> Self {
        Self(d.into())
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
