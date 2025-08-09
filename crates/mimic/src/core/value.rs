use crate::core::{
    Key,
    types::{Decimal, E8s, E18s, Principal, Subaccount, Ulid},
};
use candid::{CandidType, Principal as WrappedPrincipal};
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
    Float(f64),
    Int(i64),
    Nat(u64),
    Principal(Principal),
    Subaccount(Subaccount),
    Text(String),
    Ulid(Ulid),
    List(Vec<Box<Value>>),
    None, // specifically for Options
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
    f32 => Float,
    f64 => Float,
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
            (Self::E8s(a), Self::E8s(b)) => a.partial_cmp(b),
            (Self::E18s(a), Self::E18s(b)) => a.partial_cmp(b),
            (Self::Float(a), Self::Float(b)) => a.partial_cmp(b),
            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            (Self::Nat(a), Self::Nat(b)) => a.partial_cmp(b),
            (Self::Principal(a), Self::Principal(b)) => a.partial_cmp(b),
            (Self::Text(a), Self::Text(b)) => a.partial_cmp(b),
            (Self::Ulid(a), Self::Ulid(b)) => a.partial_cmp(b),

            // Cross-type comparisons: no ordering
            _ => None,
        }
    }
}
