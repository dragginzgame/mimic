mod bytes;
mod tests;

use crate::{
    core::{
        Key,
        traits::{FieldValue, NumFromPrimitive},
    },
    types::*,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

///
/// CONSTANTS
///

const F64_SAFE_I64: i64 = 1i64 << 53;
const F64_SAFE_U64: u64 = 1u64 << 53;
const F64_SAFE_I128: i128 = 1i128 << 53;
const F64_SAFE_U128: u128 = 1u128 << 53;

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
/// ValueEnum
/// handles the Enum case
///

#[derive(CandidType, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ValueEnum {
    pub path: String,
    pub variant: String,
}

impl ValueEnum {
    #[must_use]
    pub fn new(path: &str, variant: &str) -> Self {
        Self {
            path: path.to_string(),
            variant: variant.to_string(),
        }
    }
}

///
/// Value
/// can be used in WHERE statements
///
/// None        → the field’s value is Option::None (i.e., SQL NULL).
/// Unit        → internal placeholder for RHS; not a real value.
/// Unsupported → the field exists but isn’t filterable/indexable.
///

#[derive(CandidType, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Value {
    Account(Account),
    Blob(Vec<u8>),
    Bool(bool),
    Date(Date),
    Decimal(Decimal),
    Duration(Duration),
    Enum(ValueEnum),
    E8s(E8s),
    E18s(E18s),
    Float32(Float32),
    Float64(Float64),
    Int(i64),
    Int128(Int128),
    IntBig(Int),
    List(Vec<Value>),
    None,
    Principal(Principal),
    Subaccount(Subaccount),
    Text(String),
    Timestamp(Timestamp),
    Uint(u64),
    Uint128(Nat128),
    UintBig(Nat),
    Ulid(Ulid),
    Unit(Unit),
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
    /// TYPES
    ///

    /// Returns true if the value is one of the numeric-like variants
    /// supported by numeric comparison/ordering.
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::Decimal(_)
                | Self::Duration(_)
                | Self::E8s(_)
                | Self::E18s(_)
                | Self::Float32(_)
                | Self::Float64(_)
                | Self::Int(_)
                | Self::Int128(_)
                | Self::Timestamp(_)
                | Self::Uint(_)
                | Self::Uint128(_)
        )
    }

    /// Returns true if the value is Text.
    #[must_use]
    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    /// Returns true if the value is Unit (used for presence/null comparators).
    #[must_use]
    pub const fn is_unit(&self) -> bool {
        matches!(self, Self::Unit(_))
    }

    /// Returns true if the value is a list and all elements are Text.
    #[must_use]
    pub fn is_list_of_text(&self) -> bool {
        matches!(self, Self::List(items) if items.iter().all(Self::is_text))
    }

    ///
    /// HASHING
    ///

    #[must_use]
    pub const fn tag(&self) -> u8 {
        match self {
            Self::Account(_) => ValueTag::Account,
            Self::Blob(_) => ValueTag::Blob,
            Self::Bool(_) => ValueTag::Bool,
            Self::Date(_) => ValueTag::Date,
            Self::Decimal(_) => ValueTag::Decimal,
            Self::Duration(_) => ValueTag::Duration,
            Self::Enum(_) => ValueTag::Enum,
            Self::E8s(_) => ValueTag::E8s,
            Self::E18s(_) => ValueTag::E18s,
            Self::Float32(_) => ValueTag::Float32,
            Self::Float64(_) => ValueTag::Float64,
            Self::Int(_) => ValueTag::Int,
            Self::Int128(_) => ValueTag::Int128,
            Self::IntBig(_) => ValueTag::IntBig,
            Self::List(_) => ValueTag::List,
            Self::None => ValueTag::None,
            Self::Principal(_) => ValueTag::Principal,
            Self::Subaccount(_) => ValueTag::Subaccount,
            Self::Text(_) => ValueTag::Text,
            Self::Timestamp(_) => ValueTag::Timestamp,
            Self::Uint(_) => ValueTag::Uint,
            Self::Uint128(_) => ValueTag::Uint128,
            Self::UintBig(_) => ValueTag::UintBig,
            Self::Ulid(_) => ValueTag::Ulid,
            Self::Unit(_) => ValueTag::Unit,
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
            Self::Account(v) => Some(Key::Account(*v)),
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
            Self::Duration(d) => Decimal::from_u64(d.get()),
            Self::E8s(v) => Some(v.to_decimal()),
            Self::E18s(v) => Some(v.to_decimal()),
            Self::Float64(f) => Decimal::from_f64(f.get()),
            Self::Float32(f) => Decimal::from_f32(f.get()),
            Self::Int(i) => Decimal::from_i64(*i),
            Self::Int128(i) => Decimal::from_i128(i.get()),
            Self::Timestamp(t) => Decimal::from_u64(t.get()),
            Self::Uint(u) => Decimal::from_u64(*u),
            Self::Uint128(u) => Decimal::from_u128(u.get()),

            _ => None,
        }
    }

    // it's lossless, trust me bro
    #[allow(clippy::cast_precision_loss)]
    fn to_f64_lossless(&self) -> Option<f64> {
        match self {
            Self::Duration(d) if d.get() <= F64_SAFE_U64 => Some(d.get() as f64),
            Self::Float64(f) => Some(f.get()),
            Self::Float32(f) => Some(f64::from(f.get())),
            Self::Int(i) if (-F64_SAFE_I64..=F64_SAFE_I64).contains(i) => Some(*i as f64),
            Self::Int128(i) if (-F64_SAFE_I128..=F64_SAFE_I128).contains(&i.get()) => {
                Some(i.get() as f64)
            }
            Self::Timestamp(t) if t.get() <= F64_SAFE_U64 => Some(t.get() as f64),
            Self::Uint(u) if *u <= F64_SAFE_U64 => Some(*u as f64),
            Self::Uint128(u) if u.get() <= F64_SAFE_U128 => Some(u.get() as f64),

            _ => None,
        }
    }

    #[must_use]
    pub fn to_index_fingerprint(&self) -> Option<[u8; 16]> {
        match self {
            Self::None | Self::Unsupported => None,
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
        // NOTE: Unicode fallback — temporary to_lowercase for non‑ASCII.
        // Future: replace with proper NFKC + full casefold when available.
        std::borrow::Cow::Owned(s.to_lowercase())
    }

    #[inline]
    fn text_with_mode(s: &'_ str, mode: TextMode) -> std::borrow::Cow<'_, str> {
        match mode {
            TextMode::Cs => std::borrow::Cow::Borrowed(s),
            TextMode::Ci => Self::fold_ci(s),
        }
    }

    #[inline]
    fn text_op(
        &self,
        other: &Self,
        mode: TextMode,
        f: impl Fn(&str, &str) -> bool,
    ) -> Option<bool> {
        let (a, b) = (self.as_text()?, other.as_text()?);
        let a = Self::text_with_mode(a, mode);
        let b = Self::text_with_mode(b, mode);
        Some(f(&a, &b))
    }

    #[inline]
    fn eq_ci(a: &Self, b: &Self) -> bool {
        match (a, b) {
            (Self::Text(x), Self::Text(y)) => Self::fold_ci(x) == Self::fold_ci(y),
            _ => a == b,
        }
    }

    #[inline]
    fn normalize_list_ref(v: &Self) -> Vec<&Self> {
        match v {
            Self::List(vs) => vs.iter().collect(),
            v => vec![v],
        }
    }

    #[inline]
    fn contains_by<F>(&self, needle: &Self, eq: F) -> Option<bool>
    where
        F: Fn(&Self, &Self) -> bool,
    {
        self.as_list()
            .map(|items| items.iter().any(|v| eq(v, needle)))
    }

    #[inline]
    #[allow(clippy::unnecessary_wraps)]
    fn contains_any_by<F>(&self, needles: &Self, eq: F) -> Option<bool>
    where
        F: Fn(&Self, &Self) -> bool,
    {
        let needles = Self::normalize_list_ref(needles);
        match self {
            Self::List(items) => Some(needles.iter().any(|n| items.iter().any(|v| eq(v, n)))),
            scalar => Some(needles.iter().any(|n| eq(scalar, n))),
        }
    }

    #[inline]
    #[allow(clippy::unnecessary_wraps)]
    fn contains_all_by<F>(&self, needles: &Self, eq: F) -> Option<bool>
    where
        F: Fn(&Self, &Self) -> bool,
    {
        let needles = Self::normalize_list_ref(needles);
        match self {
            Self::List(items) => Some(needles.iter().all(|n| items.iter().any(|v| eq(v, n)))),
            scalar => Some(needles.len() == 1 && eq(scalar, needles[0])),
        }
    }

    #[inline]
    fn in_list_by<F>(&self, haystack: &Self, eq: F) -> Option<bool>
    where
        F: Fn(&Self, &Self) -> bool,
    {
        if let Self::List(items) = haystack {
            Some(items.iter().any(|h| eq(h, self)))
        } else {
            None
        }
    }

    #[must_use]
    pub fn text_eq(&self, other: &Self, mode: TextMode) -> Option<bool> {
        self.text_op(other, mode, |a, b| a == b)
    }

    #[must_use]
    pub fn text_contains(&self, needle: &Self, mode: TextMode) -> Option<bool> {
        self.text_op(needle, mode, |a, b| a.contains(b))
    }

    #[must_use]
    pub fn text_starts_with(&self, needle: &Self, mode: TextMode) -> Option<bool> {
        self.text_op(needle, mode, |a, b| a.starts_with(b))
    }

    #[must_use]
    pub fn text_ends_with(&self, needle: &Self, mode: TextMode) -> Option<bool> {
        self.text_op(needle, mode, |a, b| a.ends_with(b))
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
        self.contains_by(needle, |a, b| a == b)
    }

    #[must_use]
    pub fn contains_any(&self, needles: &Self) -> Option<bool> {
        self.contains_any_by(needles, |a, b| a == b)
    }

    #[must_use]
    pub fn contains_all(&self, needles: &Self) -> Option<bool> {
        self.contains_all_by(needles, |a, b| a == b)
    }

    #[must_use]
    pub fn in_list(&self, haystack: &Self) -> Option<bool> {
        self.in_list_by(haystack, |a, b| a == b)
    }

    #[must_use]
    pub fn contains_ci(&self, needle: &Self) -> Option<bool> {
        // Precompute folded needle and capture in comparator
        let folded_needle = match needle {
            Self::Text(b) => Some(Self::fold_ci(b)),
            _ => None,
        };
        self.contains_by(needle, |a, b| match (a, b, &folded_needle) {
            (Self::Text(x), Self::Text(_), Some(bf)) => Self::fold_ci(x) == *bf,
            _ => a == b,
        })
    }

    #[must_use]
    pub fn contains_any_ci(&self, needles: &Self) -> Option<bool> {
        self.contains_any_by(needles, Self::eq_ci)
    }

    #[must_use]
    pub fn contains_all_ci(&self, needles: &Self) -> Option<bool> {
        self.contains_all_by(needles, Self::eq_ci)
    }

    #[must_use]
    pub fn in_list_ci(&self, haystack: &Self) -> Option<bool> {
        // Precompute folded self and capture in comparator
        let folded_self = match self {
            Self::Text(b) => Some(Self::fold_ci(b)),
            _ => None,
        };
        self.in_list_by(haystack, |a, b| match (a, b, &folded_self) {
            (Self::Text(x), Self::Text(_), Some(sf)) => Self::fold_ci(x) == *sf,
            _ => a == b,
        })
    }
}

impl FieldValue for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }
}

impl From<Vec<Self>> for Value {
    fn from(vec: Vec<Self>) -> Self {
        Self::from_list(&vec)
    }
}

impl From<()> for Value {
    fn from((): ()) -> Self {
        Self::Unit(Unit)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a.partial_cmp(b),
            (Self::Date(a), Self::Date(b)) => a.partial_cmp(b),
            (Self::Decimal(a), Self::Decimal(b)) => a.partial_cmp(b),
            (Self::Duration(a), Self::Duration(b)) => a.partial_cmp(b),
            (Self::E8s(a), Self::E8s(b)) => a.partial_cmp(b),
            (Self::E18s(a), Self::E18s(b)) => a.partial_cmp(b),
            (Self::Float32(a), Self::Float32(b)) => a.partial_cmp(b),
            (Self::Float64(a), Self::Float64(b)) => a.partial_cmp(b),
            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            (Self::Int128(a), Self::Int128(b)) => a.partial_cmp(b),
            (Self::IntBig(a), Self::IntBig(b)) => a.partial_cmp(b),
            (Self::Principal(a), Self::Principal(b)) => a.partial_cmp(b),
            (Self::Subaccount(a), Self::Subaccount(b)) => a.partial_cmp(b),
            (Self::Text(a), Self::Text(b)) => a.partial_cmp(b),
            (Self::Timestamp(a), Self::Timestamp(b)) => a.partial_cmp(b),
            (Self::Uint(a), Self::Uint(b)) => a.partial_cmp(b),
            (Self::Uint128(a), Self::Uint128(b)) => a.partial_cmp(b),
            (Self::UintBig(a), Self::UintBig(b)) => a.partial_cmp(b),
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
    Account = 1,
    Blob = 2,
    Bool = 3,
    Date = 4,
    Decimal = 5,
    Duration = 6,
    Enum = 7,
    E8s = 8,
    E18s = 9,
    Float32 = 10,
    Float64 = 11,
    Int = 12,
    Int128 = 13,
    IntBig = 14,
    List = 15,
    None = 16,
    Principal = 17,
    Subaccount = 18,
    Text = 19,
    Timestamp = 20,
    Uint = 21,
    Uint128 = 22,
    UintBig = 23,
    Ulid = 24,
    Unit = 25,
    Unsupported = 26,
}

impl ValueTag {
    #[must_use]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}
