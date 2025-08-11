use crate::core::{
    Value,
    traits::{FieldValue, TypeView, ValidateAuto, ValidateCustom, Visitable},
};
use candid::CandidType;
use derive_more::Display;
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

///
/// Float32
///
/// Finite f32 only; -0.0 canonically stored as 0.0
///

#[repr(transparent)]
#[derive(CandidType, Clone, Copy, Debug, Default, Deserialize, Display, Serialize)]
pub struct Float32(f32);

impl Float32 {
    #[must_use]
    pub fn try_new(v: f32) -> Option<Self> {
        if !v.is_finite() {
            return None;
        }

        // canonicalize -0.0 → 0.0
        Some(Self(if v == 0.0 { 0.0 } else { v }))
    }

    #[must_use]
    pub fn new_clamped(v: f32) -> Self {
        Self::try_new(v).unwrap_or(Self(0.0))
    }

    #[must_use]
    pub const fn get(self) -> f32 {
        self.0
    }

    #[must_use]
    pub const fn to_be_bytes(&self) -> [u8; 4] {
        self.0.to_bits().to_be_bytes()
    }
}

impl Eq for Float32 {}

impl PartialEq for Float32 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl FieldValue for Float32 {
    fn to_value(&self) -> Value {
        Value::Float32(*self)
    }
}

impl TryFrom<f32> for Float32 {
    type Error = ();
    fn try_from(v: f32) -> Result<Self, Self::Error> {
        Self::try_new(v).ok_or(())
    }
}

impl From<Float32> for f32 {
    fn from(x: Float32) -> Self {
        x.0
    }
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
impl FromPrimitive for Float32 {
    fn from_i64(n: i64) -> Option<Self> {
        // i64 always finite in f32 (though not exact)
        Self::try_new(n as f32)
    }

    fn from_u64(n: u64) -> Option<Self> {
        Self::try_new(n as f32)
    }

    fn from_f32(n: f32) -> Option<Self> {
        Self::try_new(n)
    }

    fn from_f64(n: f64) -> Option<Self> {
        // reject out-of-range before casting
        if !n.is_finite() {
            return None;
        }
        if n < f64::from(f32::MIN) || n > f64::from(f32::MAX) {
            return None;
        }

        Self::try_new(n as f32)
    }
}

impl ToPrimitive for Float32 {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }
    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }
    fn to_f32(&self) -> Option<f32> {
        Some(self.0)
    }
    fn to_f64(&self) -> Option<f64> {
        Some(f64::from(self.0))
    }
}

impl Hash for Float32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0.to_bits()); // stable 8-byte IEEE-754
    }
}

impl Ord for Float32 {
    fn cmp(&self, other: &Self) -> Ordering {
        // safe: no NaN, -0 normalized
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl PartialOrd for Float32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TypeView for Float32 {
    type View = f32;

    fn to_view(&self) -> Self::View {
        self.0
    }

    fn from_view(view: Self::View) -> Self {
        Self(view)
    }
}

impl ValidateAuto for Float32 {}

impl ValidateCustom for Float32 {}

impl Visitable for Float32 {}

///
/// Float64
///
/// Finite f64 only; -0.0 canonically stored as 0.0
///

#[repr(transparent)]
#[derive(CandidType, Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Float64(f64);

impl Float64 {
    #[must_use]
    pub fn try_new(v: f64) -> Option<Self> {
        if !v.is_finite() {
            return None;
        }

        // canonicalize -0.0 to 0.0 so Eq/Hash/Ord are consistent
        Some(Self(if v == 0.0 { 0.0 } else { v }))
    }

    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    #[must_use]
    pub const fn to_be_bytes(&self) -> [u8; 8] {
        self.0.to_bits().to_be_bytes()
    }
}

impl Eq for Float64 {}

impl PartialEq for Float64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl FieldValue for Float64 {
    fn to_value(&self) -> Value {
        Value::Float64(*self)
    }
}

impl TryFrom<f64> for Float64 {
    type Error = ();
    fn try_from(v: f64) -> Result<Self, Self::Error> {
        Self::try_new(v).ok_or(())
    }
}

impl From<Float64> for f64 {
    fn from(x: Float64) -> Self {
        x.0
    }
}
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
impl FromPrimitive for Float64 {
    fn from_i64(n: i64) -> Option<Self> {
        Self::try_new(n as f64)
    }

    fn from_u64(n: u64) -> Option<Self> {
        Self::try_new(n as f64)
    }

    fn from_f32(n: f32) -> Option<Self> {
        Self::try_new(f64::from(n))
    }

    fn from_f64(n: f64) -> Option<Self> {
        // reject out-of-range before casting
        if !n.is_finite() {
            return None;
        }

        Self::try_new(n)
    }
}

#[allow(clippy::cast_possible_truncation)]
impl ToPrimitive for Float64 {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }
    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }
    fn to_f32(&self) -> Option<f32> {
        Some(self.0 as f32)
    }
    fn to_f64(&self) -> Option<f64> {
        Some(self.0)
    }
}

impl Hash for Float64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0.to_bits()); // stable 8-byte IEEE-754
    }
}

impl Ord for Float64 {
    fn cmp(&self, other: &Self) -> Ordering {
        // safe: no NaN, -0 normalized
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl PartialOrd for Float64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TypeView for Float64 {
    type View = f64;

    fn to_view(&self) -> Self::View {
        self.0
    }

    fn from_view(view: Self::View) -> Self {
        Self(view)
    }
}

impl ValidateAuto for Float64 {}

impl ValidateCustom for Float64 {}

impl Visitable for Float64 {}
