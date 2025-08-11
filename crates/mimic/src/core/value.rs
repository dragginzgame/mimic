use crate::core::{
    Key,
    types::{
        Account, Decimal, E8s, E18s, Float32, Float64, Int, Nat, Principal, Subaccount, Timestamp,
        Ulid,
    },
};
use candid::{CandidType, Principal as WrappedPrincipal};
use mimic_common::utils::hash::Xxh3;
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

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Value {
    Account(Account),
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
    List(Vec<Box<Value>>),
    None, // specifically for Option
    Principal(Principal),
    Subaccount(Subaccount),
    Text(String),
    Timestamp(Timestamp),
    Uint(u64),
    Ulid(Ulid),
    Unit, // when the rhs in a query doesnt matter, or the type is not filterable
}

impl Value {
    ///
    /// CONSTRUCTION
    ///

    pub fn from_list<T: Into<Self> + Clone>(items: &[T]) -> Self {
        Self::List(items.iter().cloned().map(|v| Box::new(v.into())).collect())
    }

    ///
    /// HASHING
    ///

    #[must_use]
    pub const fn tag(&self) -> u8 {
        match self {
            Self::Account(_) => ValueTag::Account,
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
    pub const fn as_list(&self) -> Option<&[Box<Self>]> {
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

    fn to_f64_lossless(&self) -> Option<f64> {
        match self {
            Self::Float64(f) => Some(f.get()),
            Self::Float32(f) => Some(f64::from(f.get())),
            Self::Int(i) if (-F64_SAFE_I..=F64_SAFE_I).contains(i) => Some(*i as f64),
            Self::Uint(u) if *u <= F64_SAFE_U => Some(*u as f64),

            _ => None,
        }
    }

    ///
    /// IS / IN
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

    #[must_use]
    pub fn in_list(&self, haystack: &Self) -> Option<bool> {
        if let Self::List(items) = haystack {
            Some(items.iter().any(|h| h.as_ref() == self))
        } else {
            None
        }
    }

    ///
    /// COMPARISON
    ///

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

    #[must_use]
    pub fn contains(&self, needle: &Self) -> Option<bool> {
        self.as_list()
            .map(|items| items.iter().any(|v| v.as_ref() == needle))
    }

    #[must_use]
    pub fn contains_any(&self, needles: &Self) -> Option<bool> {
        let (items, needles) = (self.as_list()?, needles.as_list()?);

        Some(
            needles
                .iter()
                .any(|n| items.iter().any(|v| v.as_ref() == n.as_ref())),
        )
    }

    #[must_use]
    pub fn contains_all(&self, needles: &Self) -> Option<bool> {
        let (items, needles) = (self.as_list()?, needles.as_list()?);

        Some(
            needles
                .iter()
                .all(|n| items.iter().any(|v| v.as_ref() == n.as_ref())),
        )
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
    Account = 1,
    BigInt = 2,
    BigUint = 3,
    Blob = 4,
    Bool = 5,
    Decimal = 6,
    E8s = 7,
    E18s = 8,
    Float32 = 9,
    Float64 = 10,
    Int = 11,
    List = 12,
    None = 13,
    Principal = 14,
    Subaccount = 15,
    Text = 16,
    Timestamp = 17,
    Uint = 18,
    Ulid = 19,
    Unit = 20,
}

impl ValueTag {
    #[must_use]
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
}

///
/// Canonical Byte Representation
///

#[inline]
fn feed_u8(h: &mut Xxh3, x: u8) {
    h.update(&[x]);
}
#[inline]
fn feed_u32(h: &mut Xxh3, x: u32) {
    h.update(&x.to_be_bytes());
}
#[inline]
fn feed_u64(h: &mut Xxh3, x: u64) {
    h.update(&x.to_be_bytes());
}
#[inline]
fn feed_i64(h: &mut Xxh3, x: i64) {
    h.update(&x.to_be_bytes());
}
#[inline]
fn feed_bytes(h: &mut Xxh3, b: &[u8]) {
    h.update(b);
}

#[allow(clippy::cast_possible_truncation)]
impl Value {
    fn write_to_hasher(&self, h: &mut Xxh3) {
        feed_u8(h, self.tag());

        match self {
            Self::Account(v) => {
                feed_bytes(h, v.owner_bytes());
                feed_bytes(h, &v.subaccount_bytes());
            }
            Self::BigInt(v) => {
                feed_bytes(h, &v.to_leb128());
            }
            Self::BigUint(v) => {
                feed_bytes(h, &v.to_leb128());
            }
            Self::Blob(v) => {
                feed_u8(h, 0x01);
                feed_bytes(h, v);
            }
            Self::Bool(b) => {
                feed_u8(h, u8::from(*b));
            }
            Self::Decimal(d) => {
                // encode (sign, scale, mantissa) deterministically:
                feed_u8(h, u8::from(d.is_sign_negative()));
                feed_u32(h, d.scale());
                feed_bytes(h, &d.mantissa().to_be_bytes());
            }
            Self::E8s(v) => {
                feed_u64(h, v.get());
            }
            Self::E18s(v) => {
                feed_bytes(h, &v.to_be_bytes());
            }
            Self::Float32(v) => {
                feed_bytes(h, &v.to_be_bytes());
            }
            Self::Float64(v) => {
                feed_bytes(h, &v.to_be_bytes());
            }
            Self::Int(i) => {
                feed_i64(h, *i);
            }
            Self::Principal(p) => {
                let raw = p.as_slice();
                feed_u32(h, raw.len() as u32);
                feed_bytes(h, raw);
            }
            Self::Subaccount(s) => {
                feed_bytes(h, &s.to_bytes());
            }
            Self::Text(s) => {
                // If you need case/Unicode insensitivity, normalize; else skip (much faster)
                // let norm = normalize_nfkc_casefold(s);
                // feed_u32( h, norm.len() as u32);
                // feed_bytes( h, norm.as_bytes());
                feed_u32(h, s.len() as u32);
                feed_bytes(h, s.as_bytes());
            }
            Self::Timestamp(t) => {
                feed_u64(h, t.get());
            }
            Self::Uint(u) => {
                feed_u64(h, *u);
            }
            Self::Ulid(u) => {
                feed_bytes(h, &u.to_bytes());
            }
            Self::List(xs) => {
                feed_u32(h, xs.len() as u32);
                for x in xs {
                    feed_u8(h, 0xFF);
                    x.write_to_hasher(h); // recurse, no sub-hash
                }
            }
            Self::None | Self::Unit => {}
        }
    }

    #[must_use]
    pub fn hash_value(&self) -> [u8; 16] {
        const VERSION: u8 = 1;

        let mut h = Xxh3::with_seed(0);
        feed_u8(&mut h, VERSION); // version

        self.write_to_hasher(&mut h);
        h.digest128().to_be_bytes()
    }
}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        Key,
        types::{Decimal, Float32 as F32, Float64 as F64},
    };
    use num_traits::FromPrimitive;
    use std::str::FromStr;

    // ---- helpers -----------------------------------------------------------

    fn v_f64(x: f64) -> Value {
        Value::Float64(F64::try_new(x).expect("finite f64"))
    }
    fn v_f32(x: f32) -> Value {
        Value::Float32(F32::try_new(x).expect("finite f32"))
    }
    fn v_i(x: i64) -> Value {
        Value::Int(x)
    }
    fn v_u(x: u64) -> Value {
        Value::Uint(x)
    }
    fn v_d_i(x: i64) -> Value {
        Value::Decimal(Decimal::from_i64(x).unwrap())
    }
    fn v_txt(s: &str) -> Value {
        Value::Text(s.to_string())
    }

    // ---- hashing -----------------------------------------------------------

    #[test]
    fn hash_is_deterministic_for_int() {
        let v = Value::Int(42);
        let a = v.hash_value();
        let b = v.hash_value();
        assert_eq!(a, b, "hash should be deterministic for same value");
    }

    #[test]
    fn different_variants_produce_different_hashes() {
        let a = Value::Int(5).hash_value();
        let b = Value::Uint(5).hash_value();
        assert_ne!(
            a, b,
            "Int(5) and Uint(5) must hash differently (different tag)"
        );
    }

    #[test]
    fn float32_and_float64_hash_differ() {
        let a = v_f32(1.0).hash_value();
        let b = v_f64(1.0).hash_value();
        assert_ne!(
            a, b,
            "Float32 and Float64 must hash differently (different tag)"
        );
    }

    #[test]
    fn text_is_length_and_content_sensitive() {
        let a = v_txt("foo").hash_value();
        let b = v_txt("bar").hash_value();
        assert_ne!(a, b, "different strings should hash differently");

        let c = v_txt("foo").hash_value();
        assert_eq!(a, c, "same string should hash the same");
    }

    #[test]
    fn list_hash_is_order_sensitive() {
        let l1 = Value::from_list(&[v_i(1), v_i(2)]);
        let l2 = Value::from_list(&[v_i(2), v_i(1)]);
        assert_ne!(
            l1.hash_value(),
            l2.hash_value(),
            "list order should affect hash"
        );
    }

    #[test]
    fn list_hash_is_length_sensitive() {
        let l1 = Value::from_list(&[v_i(1)]);
        let l2 = Value::from_list(&[v_i(1), v_i(1)]);
        assert_ne!(
            l1.hash_value(),
            l2.hash_value(),
            "list length should affect hash"
        );
    }

    // ---- keys --------------------------------------------------------------

    #[test]
    fn as_key_some_for_orderable_variants() {
        assert_eq!(Value::Int(7).as_key(), Some(Key::Int(7)));
        assert_eq!(Value::Uint(7).as_key(), Some(Key::Uint(7)));
        assert_eq!(Value::Ulid(Ulid::MIN).as_key(), Some(Key::Ulid(Ulid::MIN)));
        // Non-orderable / non-key variants
        assert!(v_txt("x").as_key().is_none());
        assert!(Value::Decimal(Decimal::new(1, 0)).as_key().is_none());
        assert!(Value::List(vec![]).as_key().is_none());
        assert!(Value::None.as_key().is_none());
    }

    #[test]
    fn from_key_round_trips() {
        let ks = [Key::Int(-9), Key::Uint(9), Key::Ulid(Ulid::MAX)];
        for k in ks {
            let v = Value::from(k);
            let back = v
                .as_key()
                .expect("as_key should succeed for orderable variants");
            assert_eq!(
                k, back,
                "Value <-> Key round trip failed: {k:?} -> {v:?} -> {back:?}"
            );
        }
    }

    // ---- numeric coercion & comparison ------------------------------------

    #[test]
    fn cmp_numeric_int_nat_eq_and_order() {
        assert_eq!(v_i(10).cmp_numeric(&v_u(10)), Some(Ordering::Equal));
        assert_eq!(v_i(9).cmp_numeric(&v_u(10)), Some(Ordering::Less));
        // negative int vs nat: not comparable via f64 path; decimal path handles it
        assert_eq!(v_i(-1).cmp_numeric(&v_u(0)), Some(Ordering::Less));
    }

    #[test]
    fn cmp_numeric_int_float_eq() {
        assert_eq!(v_i(42).cmp_numeric(&v_f64(42.0)), Some(Ordering::Equal));
        assert_eq!(v_i(42).cmp_numeric(&v_f32(42.0)), Some(Ordering::Equal));
    }

    #[test]
    fn cmp_numeric_decimal_int_and_float() {
        assert_eq!(v_d_i(10).cmp_numeric(&v_i(10)), Some(Ordering::Equal));
        assert_eq!(v_d_i(10).cmp_numeric(&v_f64(10.0)), Some(Ordering::Equal));
        assert_eq!(v_d_i(11).cmp_numeric(&v_f64(10.5)), Some(Ordering::Greater));
    }

    #[test]
    fn cmp_numeric_safe_int_boundary() {
        // 2^53 is exactly representable in f64
        let safe: i64 = 9_007_199_254_740_992; // 1 << 53
        let int_safe = v_i(safe);
        let float_safe = v_f64(safe as f64);
        assert_eq!(int_safe.cmp_numeric(&float_safe), Some(Ordering::Equal));

        // one above 2^53 is not exactly representable; decimal path should see it as greater
        let int_unsafe = v_i(safe + 1);
        assert_eq!(int_unsafe.cmp_numeric(&float_safe), Some(Ordering::Greater));
    }

    #[test]
    fn cmp_numeric_neg_zero_equals_zero() {
        let neg_zero = Value::Float64(F64::try_new(-0.0).unwrap());
        assert_eq!(neg_zero.cmp_numeric(&v_i(0)), Some(Ordering::Equal));
        let neg_zero32 = Value::Float32(F32::try_new(-0.0).unwrap());
        assert_eq!(neg_zero32.cmp_numeric(&v_i(0)), Some(Ordering::Equal));
    }

    #[test]
    fn partial_ord_cross_variant_is_none() {
        // PartialOrd stays within same variant; cross-variant returns None
        assert!(v_i(1).partial_cmp(&v_f64(1.0)).is_none());
        assert!(v_txt("a").partial_cmp(&v_txt("b")).is_some());
    }

    // ---- list membership ---------------------------------------------------

    #[test]
    fn list_contains_scalar() {
        let l = Value::from_list(&[v_i(1), v_txt("a")]);
        assert_eq!(l.contains(&v_i(1)), Some(true));
        assert_eq!(l.contains(&v_i(2)), Some(false));
    }

    #[test]
    fn list_contains_any_all_and_vacuous_truth() {
        let l = Value::from_list(&[v_txt("x"), v_txt("y")]);
        let needles_any = Value::from_list(&[v_txt("z"), v_txt("y")]);
        let needles_all = Value::from_list(&[v_txt("x"), v_txt("y")]);
        let empty = Value::from_list::<Value>(&[]);
        assert_eq!(l.contains_any(&needles_any), Some(true));
        assert_eq!(l.contains_all(&needles_all), Some(true));
        assert_eq!(l.contains_any(&empty), Some(false), "AnyIn([]) == false");
        assert_eq!(l.contains_all(&empty), Some(true), "AllIn([]) == true");
    }

    // ---- text CS/CI --------------------------------------------------------

    #[test]
    fn text_eq_cs_vs_ci() {
        let a = v_txt("Alpha");
        let b = v_txt("alpha");
        assert_eq!(a.text_eq(&b, TextMode::Cs), Some(false));
        assert_eq!(a.text_eq(&b, TextMode::Ci), Some(true));
    }

    #[test]
    fn text_contains_starts_ends_cs_ci() {
        let a = v_txt("Hello Alpha World");
        assert_eq!(a.text_contains(&v_txt("alpha"), TextMode::Cs), Some(false));
        assert_eq!(a.text_contains(&v_txt("alpha"), TextMode::Ci), Some(true));

        assert_eq!(
            a.text_starts_with(&v_txt("hello"), TextMode::Cs),
            Some(false)
        );
        assert_eq!(
            a.text_starts_with(&v_txt("hello"), TextMode::Ci),
            Some(true)
        );

        assert_eq!(a.text_ends_with(&v_txt("WORLD"), TextMode::Cs), Some(false));
        assert_eq!(a.text_ends_with(&v_txt("WORLD"), TextMode::Ci), Some(true));
    }

    // ---- E8s / E18s <-> Decimal / Float cross-type tests -------------------

    // helper constructors — ADAPT these to your actual API
    fn v_e8(raw: u64) -> Value {
        // e.g., E8s::from_raw(raw) or E8s(raw)
        Value::E8s(E8s::from(raw)) // <-- change if needed
    }
    fn v_e18(raw: u128) -> Value {
        Value::E18s(E18s::from(raw)) // <-- change if needed
    }
    fn v_dec_str(s: &str) -> Value {
        Value::Decimal(Decimal::from_str(s).expect("valid decimal"))
    }

    #[test]
    fn e8s_equals_decimal_when_scaled() {
        // 1.00 token == 100_000_000 e8s
        let one_token_e8s = v_e8(100_000_000);
        let one_token_dec = v_dec_str("1");
        assert_eq!(
            one_token_e8s.cmp_numeric(&one_token_dec),
            Some(Ordering::Equal)
        );

        // 12.34567890 tokens == 1_234_567_890 e8s
        let e8s = v_e8(1_234_567_890);
        let dec = v_dec_str("12.3456789");
        assert_eq!(e8s.cmp_numeric(&dec), Some(Ordering::Equal));
    }

    #[test]
    fn e8s_orders_correctly_against_decimal() {
        let nine_tenths_e8s = v_e8(90_000_000);
        let one_dec = v_dec_str("1");
        assert_eq!(nine_tenths_e8s.cmp_numeric(&one_dec), Some(Ordering::Less));

        let eleven_tenths_e8s = v_e8(110_000_000);
        assert_eq!(
            eleven_tenths_e8s.cmp_numeric(&one_dec),
            Some(Ordering::Greater)
        );
    }

    #[test]
    fn e8s_vs_float64_safe_eq() {
        // 2^53-safe region: exact in f64 when converted through Decimal or safe-int path
        let e8s = v_e8(200_000_000); // 2.0
        assert_eq!(e8s.cmp_numeric(&v_f64(2.0)), Some(Ordering::Equal));
    }

    #[test]
    fn e18s_equals_decimal_when_scaled() {
        // 1.000000000000000000 == 1e18 e18s
        let one = v_e18(1_000_000_000_000_000_000);
        let one_dec = v_dec_str("1");
        assert_eq!(one.cmp_numeric(&one_dec), Some(Ordering::Equal));

        // 0.000000000000000123 == 123 e18s
        let tiny = v_e18(123);
        let tiny_dec = v_dec_str("0.000000000000000123");
        assert_eq!(tiny.cmp_numeric(&tiny_dec), Some(Ordering::Equal));
    }

    #[test]
    fn e18s_ordering_and_float_cross_check() {
        let half = v_e18(500_000_000_000_000_000); // 0.5
        assert_eq!(half.cmp_numeric(&v_dec_str("0.4")), Some(Ordering::Greater));
        assert_eq!(half.cmp_numeric(&v_dec_str("0.6")), Some(Ordering::Less));
        assert_eq!(half.cmp_numeric(&v_f64(0.5)), Some(Ordering::Equal));
    }

    #[test]
    fn e8s_e18s_text_and_list_do_not_compare() {
        // sanity: non-numeric shapes return None from cmp_numeric
        assert!(v_e8(1).partial_cmp(&v_txt("1")).is_none());
        assert!(v_e18(1).partial_cmp(&Value::from_list(&[v_i(1)])).is_none());
    }
}
