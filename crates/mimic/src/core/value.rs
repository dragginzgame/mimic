use crate::core::{
    Key,
    types::{Decimal, E8s, E18s, Float32, Float64, Principal, Subaccount, Ulid},
};
use candid::{CandidType, Principal as WrappedPrincipal};
use mimic_common::utils::hash::Xxh3;
use serde::{Deserialize, Serialize};

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
/// can be searched or used in WHERE statements
///

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Value {
    Bool(bool),
    Decimal(Decimal),
    E8s(E8s),
    E18s(E18s),
    Float32(Float32),
    Float64(Float64),
    Int(i64),
    Nat(u64),
    Principal(Principal),
    Subaccount(Subaccount),
    Text(String),
    Ulid(Ulid),
    List(Vec<Box<Value>>),
    None, // specifically for Option
    Unsupported,
}

impl Value {
    #[must_use]
    pub const fn as_key(&self) -> Option<Key> {
        match self {
            Self::Int(v) => Some(Key::Int(*v)),
            Self::Nat(v) => Some(Key::Nat(*v)),
            Self::Principal(v) => Some(Key::Principal(*v)),
            Self::Subaccount(v) => Some(Key::Subaccount(*v)),
            Self::Ulid(v) => Some(Key::Ulid(*v)),
            _ => None,
        }
    }

    /// Return the unmodified searchable string
    #[must_use]
    pub fn to_searchable_string(&self) -> Option<String> {
        match self {
            Self::Decimal(v) => Some(v.to_string()),
            Self::Principal(v) => Some(v.to_text()),
            Self::Text(v) => Some(v.to_string()),
            Self::Ulid(v) => Some(v.to_string()),
            _ => None,
        }
    }

    // list
    pub fn list<T: Into<Self> + Clone>(items: &[T]) -> Self {
        Self::List(items.iter().cloned().map(|v| Box::new(v.into())).collect())
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
    Ulid => Ulid,
    u8 => Nat,
    u16 => Nat,
    u32 => Nat,
    u64 => Nat,
}

impl From<Key> for Value {
    fn from(value: Key) -> Self {
        match value {
            Key::Invalid => Self::Unsupported,
            Key::Int(v) => Self::Int(v),
            Key::Nat(v) => Self::Nat(v),
            Key::Principal(v) => Self::Principal(v),
            Key::Subaccount(v) => Self::Subaccount(v),
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
        Self::List(vec.into_iter().map(Box::new).collect())
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a.partial_cmp(b),
            (Self::Decimal(a), Self::Decimal(b)) => a.partial_cmp(b),
            (Self::E8s(a), Self::E8s(b)) => a.partial_cmp(b),
            (Self::E18s(a), Self::E18s(b)) => a.partial_cmp(b),
            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            (Self::Nat(a), Self::Nat(b)) => a.partial_cmp(b),
            (Self::Principal(a), Self::Principal(b)) => a.partial_cmp(b),
            (Self::Subaccount(a), Self::Subaccount(b)) => a.partial_cmp(b),
            (Self::Text(a), Self::Text(b)) => a.partial_cmp(b),
            (Self::Ulid(a), Self::Ulid(b)) => a.partial_cmp(b),

            // Cross-type comparisons: no ordering
            _ => None,
        }
    }
}

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
        match self {
            Self::Bool(b) => {
                feed_u8(h, 0x01);
                feed_u8(h, u8::from(*b));
            }
            Self::Decimal(d) => {
                feed_u8(h, 0x02);
                // encode (sign, scale, mantissa) deterministically:
                feed_u8(h, u8::from(d.is_sign_negative()));
                feed_u32(h, d.scale());
                feed_bytes(h, &d.mantissa().to_be_bytes());
            }
            Self::E8s(v) => {
                feed_u8(h, 0x03);
                feed_u64(h, v.into_inner());
            }
            Self::E18s(v) => {
                feed_u8(h, 0x04);
                feed_bytes(h, &v.to_be_bytes());
            }
            Self::Float32(v) => {
                feed_u8(h, 0x04);
                feed_bytes(h, &v.to_be_bytes());
            }
            Self::Float64(v) => {
                feed_u8(h, 0x04);
                feed_bytes(h, &v.to_be_bytes());
            }
            Self::Int(i) => {
                feed_u8(h, 0x05);
                feed_i64(h, *i);
            }
            Self::Nat(u) => {
                feed_u8(h, 0x06);
                feed_u64(h, *u);
            }
            Self::Principal(p) => {
                feed_u8(h, 0x07);
                let raw = p.as_slice();
                feed_u32(h, raw.len() as u32);
                feed_bytes(h, raw);
            }
            Self::Subaccount(s) => {
                feed_u8(h, 0x08);
                feed_bytes(h, &s.to_bytes());
            } // assuming &[u8; 32]
            Self::Text(s) => {
                feed_u8(h, 0x09);
                // If you need case/Unicode insensitivity, normalize; else skip (much faster)
                // let norm = normalize_nfkc_casefold(s);
                // feed_u32( h, norm.len() as u32);
                // feed_bytes( h, norm.as_bytes());
                feed_u32(h, s.len() as u32);
                feed_bytes(h, s.as_bytes());
            }
            Self::Ulid(u) => {
                feed_u8(h, 0x0A);
                feed_bytes(h, &u.to_bytes());
            }
            Self::List(xs) => {
                feed_u8(h, 0x0B);
                feed_u32(h, xs.len() as u32);
                for x in xs {
                    feed_u8(h, 0xFF);
                    x.write_to_hasher(h); // recurse, no sub-hash
                }
            }
            Self::None => {
                feed_u8(h, 0x0C);
            }
            Self::Unsupported => {
                feed_u8(h, 0x0D);
            }
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
    use crate::core::{Key, types::Decimal};

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
        let b = Value::Nat(5).hash_value();

        assert_ne!(
            a, b,
            "Int(5) and Nat(5) must hash differently (different tag)"
        );
    }

    #[test]
    fn text_is_length_and_content_sensitive() {
        let a = Value::Text("foo".to_string()).hash_value();
        let b = Value::Text("bar".to_string()).hash_value();
        assert_ne!(a, b, "different strings should hash differently");

        let c = Value::Text("foo".to_string()).hash_value();
        assert_eq!(a, c, "same string should hash the same");
    }

    #[test]
    fn list_hash_is_order_sensitive() {
        let l1 = Value::list(&[Value::Int(1), Value::Int(2)]);
        let l2 = Value::list(&[Value::Int(2), Value::Int(1)]);

        assert_ne!(
            l1.hash_value(),
            l2.hash_value(),
            "list order should affect hash"
        );
    }

    #[test]
    fn list_hash_is_length_sensitive() {
        let l1 = Value::list(&[Value::Int(1)]);
        let l2 = Value::list(&[Value::Int(1), Value::Int(1)]);

        assert_ne!(
            l1.hash_value(),
            l2.hash_value(),
            "list length should affect hash"
        );
    }

    #[test]
    fn as_key_some_for_orderable_variants() {
        assert_eq!(Value::Int(7).as_key(), Some(Key::Int(7)));
        assert_eq!(Value::Nat(7).as_key(), Some(Key::Nat(7)));
        assert_eq!(Value::Ulid(Ulid::MIN).as_key(), Some(Key::Ulid(Ulid::MIN)));
        // Non-orderable / non-key variants
        assert!(Value::Text("x".into()).as_key().is_none());
        assert!(Value::Decimal(Decimal::new(1, 0)).as_key().is_none());
        assert!(Value::List(vec![]).as_key().is_none());
        assert!(Value::None.as_key().is_none());
        assert!(Value::Unsupported.as_key().is_none());
    }

    #[test]
    fn from_key_round_trips() {
        let ks = [Key::Int(-9), Key::Nat(9), Key::Ulid(Ulid::MAX)];
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
}
