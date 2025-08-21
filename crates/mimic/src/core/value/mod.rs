mod bytes;
mod tests;

use crate::core::{
    Key,
    types::{
        Decimal, E8s, E18s, Float32, Float64, Int, Nat, Principal, Subaccount, Timestamp, Ulid,
    },
};
use candid::{CandidType, Principal as WrappedPrincipal};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

///
/// CONSTANTS
///

const F64_SAFE_I: i64 = 1i64 << 53;
const F64_SAFE_U: u64 = 1u64 << 53;

///
/// TextMode
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextMode {
    Cs, // case-sensitive
    Ci, // case-insensitive
}

///
/// Handy Macros
///

#[macro_export]
macro_rules! impl_from_for {
    ( $struct:ty, $( $type:ty => $variant:ident ),* $(,)? ) => {
        $(
            impl From<$type> for $struct {
                fn from(v: $type) -> Self {
                    Self::$variant(v.into())
                }
            }
        )*
    };
}

///
/// Value
/// can be used in WHERE statements
///
/// Cheatsheet
///
/// None        → the field’s value is Option::None (i.e., SQL NULL).
/// Unit        → internal placeholder for RHS; not a real value.
/// Unsupported → the field exists but isn’t filterable/indexable.
///

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Value {
    BigInt(Int),
    BigUint(Nat),
    Blob(Vec<u8>),
    Bool(bool),
    Decimal(Decimal),
    E8s(E8s),
    E18s(E18s),
    Float32(Float32),
    Float64(Float64),
    Int(i64),
    List(Vec<Value>),
    None,
    Principal(Principal),
    Subaccount(Subaccount),
    Text(String),
    Timestamp(Timestamp),
    Uint(u64),
    Ulid(Ulid),
    Unit,
    Unsupported,
}

impl Value {
    ///
    /// CONSTRUCTION
    ///

    pub fn from_list<T: Into<Self> + Clone>(items: &[T]) -> Self {
        Self::List(items.iter().cloned().map(Into::into).collect())
    }

    ///
    /// HASHING
    ///

    #[must_use]
    pub const fn tag(&self) -> u8 {
        match self {
            Self::BigInt(_) => ValueTag::BigInt,
            Self::BigUint(_) => ValueTag::BigUint,
            Self::Blob(_) => ValueTag::Blob,
            Self::Bool(_) => ValueTag::Bool,
            Self::Decimal(_) => ValueTag::Decimal,
            Self::E8s(_) => ValueTag::E8s,
            Self::E18s(_) => ValueTag::E18s,
            Self::Float32(_) => ValueTag::Float32,
            Self::Float64(_) => ValueTag::Float64,
            Self::Int(_) => ValueTag::Int,
            Self::List(_) => ValueTag::List,
            Self::None => ValueTag::None,
            Self::Principal(_) => ValueTag::Principal,
            Self::Subaccount(_) => ValueTag::Subaccount,
            Self::Text(_) => ValueTag::Text,
            Self::Timestamp(_) => ValueTag::Timestamp,
            Self::Uint(_) => ValueTag::Uint,
            Self::Ulid(_) => ValueTag::Ulid,
            Self::Unit => ValueTag::Unit,
            Self::Unsupported => ValueTag::Unsupported,
        }
        .to_u8()
    }

    ///
    /// CONVERSION
    ///

    #[must_use]
    pub const fn as_key(&self) -> Option<Key> {
        match self {
            Self::Int(v) => Some(Key::Int(*v)),
            Self::Uint(v) => Some(Key::Uint(*v)),
            Self::Principal(v) => Some(Key::Principal(*v)),
            Self::Subaccount(v) => Some(Key::Subaccount(*v)),
            Self::Ulid(v) => Some(Key::Ulid(*v)),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_text(&self) -> Option<&str> {
        if let Self::Text(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_list(&self) -> Option<&[Self]> {
        if let Self::List(xs) = self {
            Some(xs.as_slice())
        } else {
            None
        }
    }

    fn to_decimal(&self) -> Option<Decimal> {
        match self {
            Self::Decimal(d) => Some(*d),
            Self::E8s(v) => Some(v.to_decimal()),
            Self::E18s(v) => Some(v.to_decimal()),
            Self::Float64(f) => Decimal::from_f64(f.get()),
            Self::Float32(f) => Decimal::from_f32(f.get()),
            Self::Int(i) => Decimal::from_i64(*i),
            Self::Timestamp(t) => Decimal::from_u64(t.get()),
            Self::Uint(u) => Decimal::from_u64(*u),

            _ => None,
        }
    }

    // it's lossless, trust me bro
    #[allow(clippy::cast_precision_loss)]
    fn to_f64_lossless(&self) -> Option<f64> {
        match self {
            Self::Float64(f) => Some(f.get()),
            Self::Float32(f) => Some(f64::from(f.get())),
            Self::Int(i) if (-F64_SAFE_I..=F64_SAFE_I).contains(i) => Some(*i as f64),
            Self::Uint(u) if *u <= F64_SAFE_U => Some(*u as f64),

            _ => None,
        }
    }

    #[must_use]
    pub fn to_index_fingerprint(&self) -> Option<[u8; 16]> {
        match self {
            Self::None | Self::Unit | Self::Unsupported => None,
            _ => Some(self.hash_value()),
        }
    }

    /// Cross-type numeric comparison; returns None if non-numeric.
    #[must_use]
    pub fn cmp_numeric(&self, other: &Self) -> Option<Ordering> {
        if let (Some(a), Some(b)) = (self.to_decimal(), other.to_decimal()) {
            return a.partial_cmp(&b);
        }
        if let (Some(a), Some(b)) = (self.to_f64_lossless(), other.to_f64_lossless()) {
            return a.partial_cmp(&b);
        }
        None
    }

    ///
    /// TEXT COMPARISON
    ///

    #[inline]
    fn fold_ci(s: &str) -> std::borrow::Cow<'_, str> {
        if s.is_ascii() {
            return std::borrow::Cow::Owned(s.to_ascii_lowercase());
        }
        // TODO: swap to proper NFKC+casefold helper when you add it
        std::borrow::Cow::Owned(s.to_lowercase())
    }

    #[must_use]
    pub fn text_eq(&self, other: &Self, mode: TextMode) -> Option<bool> {
        let (a, b) = (self.as_text()?, other.as_text()?);

        Some(match mode {
            TextMode::Cs => a == b,
            TextMode::Ci => Self::fold_ci(a) == Self::fold_ci(b),
        })
    }

    #[must_use]
    pub fn text_contains(&self, needle: &Self, mode: TextMode) -> Option<bool> {
        let (a, b) = (self.as_text()?, needle.as_text()?);

        Some(match mode {
            TextMode::Cs => a.contains(b),
            TextMode::Ci => Self::fold_ci(a).contains(&*Self::fold_ci(b)),
        })
    }

    #[must_use]
    pub fn text_starts_with(&self, needle: &Self, mode: TextMode) -> Option<bool> {
        let (a, b) = (self.as_text()?, needle.as_text()?);

        Some(match mode {
            TextMode::Cs => a.starts_with(b),
            TextMode::Ci => Self::fold_ci(a).starts_with(&*Self::fold_ci(b)),
        })
    }

    #[must_use]
    pub fn text_ends_with(&self, needle: &Self, mode: TextMode) -> Option<bool> {
        let (a, b) = (self.as_text()?, needle.as_text()?);

        Some(match mode {
            TextMode::Cs => a.ends_with(b),
            TextMode::Ci => Self::fold_ci(a).ends_with(&*Self::fold_ci(b)),
        })
    }

    ///
    /// EMPTY
    ///

    #[must_use]
    pub const fn is_empty(&self) -> Option<bool> {
        match self {
            Self::List(xs) => Some(xs.is_empty()),
            Self::Text(s) => Some(s.is_empty()),
            Self::Blob(b) => Some(b.is_empty()),
            _ => None, // no concept of "empty"
        }
    }

    #[must_use]
    pub fn is_not_empty(&self) -> Option<bool> {
        self.is_empty().map(|b| !b)
    }

    ///
    /// COLLECTIONS
    ///

    #[must_use]
    pub fn contains(&self, needle: &Self) -> Option<bool> {
        self.as_list()
            .map(|items| items.iter().any(|v| v == needle))
    }

    #[must_use]
    pub fn contains_any(&self, needles: &Self) -> Option<bool> {
        // normalize RHS → list
        let needles: Vec<&Self> = match needles {
            Self::List(vs) => vs.iter().collect(),
            v => vec![v],
        };

        match self {
            // Case 1: actual is a list → check any overlap
            Self::List(items) => Some(needles.iter().any(|n| items.iter().any(|v| v == *n))),

            // Case 2: actual is scalar → does it appear in RHS list?
            scalar => Some(needles.contains(&scalar)),
        }
    }

    #[must_use]
    pub fn contains_all(&self, needles: &Self) -> Option<bool> {
        let needles: Vec<&Self> = match needles {
            Self::List(vs) => vs.iter().collect(),
            v => vec![v],
        };

        match self {
            // Case 1: actual is a list → does it contain all RHS?
            Self::List(items) => Some(needles.iter().all(|n| items.iter().any(|v| v == *n))),

            // Case 2: actual is scalar → only true if RHS is exactly one matching element
            scalar => Some(needles.len() == 1 && needles[0] == scalar),
        }
    }

    #[must_use]
    pub fn in_list(&self, haystack: &Self) -> Option<bool> {
        if let Self::List(items) = haystack {
            Some(items.iter().any(|h| h == self))
        } else {
            None
        }
    }

    #[must_use]
    pub fn contains_ci(&self, needle: &Self) -> Option<bool> {
        self.as_list().map(|items| {
            items.iter().any(|v| match (v, needle) {
                (Self::Text(a), Self::Text(b)) => Self::fold_ci(a) == Self::fold_ci(b),
                _ => v == needle,
            })
        })
    }

    #[must_use]
    pub fn contains_any_ci(&self, needles: &Self) -> Option<bool> {
        // normalize RHS → list
        let needles: Vec<&Self> = match needles {
            Self::List(vs) => vs.iter().collect(),
            v => vec![v],
        };

        match self {
            // Case 1: actual is a list → check any overlap
            Self::List(items) => Some(needles.iter().any(|n| {
                items.iter().any(|v| match (v, *n) {
                    (Self::Text(a), Self::Text(b)) => Self::fold_ci(a) == Self::fold_ci(b),
                    _ => v == *n,
                })
            })),

            // Case 2: actual is scalar → does it appear in RHS list?
            scalar => Some(needles.iter().any(|n| match (scalar, *n) {
                (Self::Text(a), Self::Text(b)) => Self::fold_ci(a) == Self::fold_ci(b),
                _ => scalar == *n,
            })),
        }
    }

    #[must_use]
    pub fn contains_all_ci(&self, needles: &Self) -> Option<bool> {
        let needles: Vec<&Self> = match needles {
            Self::List(vs) => vs.iter().collect(),
            v => vec![v],
        };

        match self {
            // Case 1: actual is a list → does it contain all RHS?
            Self::List(items) => Some(needles.iter().all(|n| {
                items.iter().any(|v| match (v, *n) {
                    (Self::Text(a), Self::Text(b)) => Self::fold_ci(a) == Self::fold_ci(b),
                    _ => v == *n,
                })
            })),

            // Case 2: actual is scalar → only true if RHS is exactly one matching element
            scalar => Some(
                needles.len() == 1
                    && match (scalar, needles[0]) {
                        (Self::Text(a), Self::Text(b)) => Self::fold_ci(a) == Self::fold_ci(b),
                        _ => scalar == needles[0],
                    },
            ),
        }
    }

    #[must_use]
    pub fn in_list_ci(&self, haystack: &Self) -> Option<bool> {
        if let Self::List(items) = haystack {
            Some(items.iter().any(|h| match (h, self) {
                (Self::Text(a), Self::Text(b)) => Self::fold_ci(a) == Self::fold_ci(b),
                _ => h == self,
            }))
        } else {
            None
        }
    }
}

impl_from_for! {
    Value,
    bool => Bool,
    Decimal => Decimal,
    E8s => E8s,
    E18s => E18s,
    i8 => Int,
    i16 => Int,
    i32 => Int,
    i64 => Int,
    Principal => Principal,
    &str => Text,
    String => Text,
    Timestamp => Timestamp,
    Ulid => Ulid,
    u8 => Uint,
    u16 => Uint,
    u32 => Uint,
    u64 => Uint,
}

// Infallible: every Key can be represented as a Value
impl From<Key> for Value {
    fn from(k: Key) -> Self {
        match k {
            Key::Int(v) => Self::Int(v),
            Key::Principal(v) => Self::Principal(v),
            Key::Subaccount(v) => Self::Subaccount(v),
            Key::Timestamp(v) => Self::Timestamp(v),
            Key::Uint(v) => Self::Uint(v),
            Key::Ulid(v) => Self::Ulid(v),
        }
    }
}

impl From<&Key> for Value {
    fn from(value: &Key) -> Self {
        (*value).into()
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        (value.clone()).into()
    }
}

impl From<&Ulid> for Value {
    fn from(value: &Ulid) -> Self {
        (*value).into()
    }
}

impl From<WrappedPrincipal> for Value {
    fn from(v: WrappedPrincipal) -> Self {
        Self::Principal(v.into())
    }
}

impl From<Vec<Self>> for Value {
    fn from(vec: Vec<Self>) -> Self {
        Self::from_list(&vec)
    }
}

impl From<()> for Value {
    fn from((): ()) -> Self {
        Self::Unit
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::BigInt(a), Self::BigInt(b)) => a.partial_cmp(b),
            (Self::BigUint(a), Self::BigUint(b)) => a.partial_cmp(b),
            (Self::Bool(a), Self::Bool(b)) => a.partial_cmp(b),
            (Self::Decimal(a), Self::Decimal(b)) => a.partial_cmp(b),
            (Self::E8s(a), Self::E8s(b)) => a.partial_cmp(b),
            (Self::E18s(a), Self::E18s(b)) => a.partial_cmp(b),
            (Self::Float32(a), Self::Float32(b)) => a.partial_cmp(b),
            (Self::Float64(a), Self::Float64(b)) => a.partial_cmp(b),
            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            (Self::Principal(a), Self::Principal(b)) => a.partial_cmp(b),
            (Self::Subaccount(a), Self::Subaccount(b)) => a.partial_cmp(b),
            (Self::Text(a), Self::Text(b)) => a.partial_cmp(b),
            (Self::Timestamp(a), Self::Timestamp(b)) => a.partial_cmp(b),
            (Self::Uint(a), Self::Uint(b)) => a.partial_cmp(b),
            (Self::Ulid(a), Self::Ulid(b)) => a.partial_cmp(b),

            // Cross-type comparisons: no ordering
            _ => None,
        }
    }
}

///
/// ValueTag
///

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueTag {
    BigInt = 1,
    BigUint = 2,
    Blob = 3,
    Bool = 4,
    Decimal = 5,
    E8s = 6,
    E18s = 7,
    Float32 = 8,
    Float64 = 9,
    Int = 10,
    List = 11,
    None = 12,
    Principal = 13,
    Subaccount = 14,
    Text = 15,
    Timestamp = 16,
    Uint = 17,
    Ulid = 18,
    Unit = 19,
    Unsupported = 20,
}

impl ValueTag {
    #[must_use]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
