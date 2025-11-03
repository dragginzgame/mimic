#[macro_use]
mod macros;
mod sanitize;
mod validate;
mod visitable;

pub use sanitize::*;
pub use validate::*;
pub use visitable::*;

// re-exports of other traits
// for the standard traits::X pattern
pub use canic::cdk::structures::storable::Storable;
pub use num_traits::{FromPrimitive as NumFromPrimitive, NumCast, ToPrimitive as NumToPrimitive};
pub use serde::{Deserialize, Serialize, de::DeserializeOwned};
pub use std::{
    cmp::{Eq, Ordering, PartialEq},
    convert::{AsRef, From, Into},
    default::Default,
    fmt::{Debug, Display},
    hash::Hash,
    iter::IntoIterator,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::{
    core::{Key, Value},
    db::query::FilterExpr,
    schema::node::Index,
};

/// ------------------------
/// KIND TRAITS
/// the Schema uses the term "Node" but when they're built it's "Kind"
/// ------------------------

///
/// Kind
///

pub trait Kind: Path + 'static {}

impl<T> Kind for T where T: Path + 'static {}

///
/// CanisterKind
///

pub trait CanisterKind: Kind {}

///
/// EntityKind
///

pub trait EntityKind: Kind + TypeKind + FieldValues {
    type PrimaryKey: Copy + Into<Key>;
    type Store: StoreKind;
    type Canister: CanisterKind; // Self::Store::Canister shortcut

    const ENTITY_ID: u64;
    const PRIMARY_KEY: &'static str;
    const FIELDS: &'static [&'static str];
    const INDEXES: &'static [&'static Index];

    fn key(&self) -> Key;
    fn primary_key(&self) -> Self::PrimaryKey;
}

///
/// StoreKind
///

pub trait StoreKind: Kind {
    type Canister: CanisterKind;
}

/// ------------------------
/// TYPE TRAITS
/// ------------------------

///
/// TypeKind
/// any data type
///

pub trait TypeKind:
    Kind
    + View
    + Clone
    + Default
    + Serialize
    + DeserializeOwned
    + Sanitize
    + Validate
    + Visitable
    + PartialEq
{
}

impl<T> TypeKind for T where
    T: Kind
        + View
        + Clone
        + Default
        + DeserializeOwned
        + PartialEq
        + Serialize
        + Sanitize
        + Validate
        + Visitable
{
}

/// ------------------------
/// VIEW TRAITS
/// ------------------------

///
/// View
///

pub trait View {
    type ViewType: Default;

    fn to_view(&self) -> Self::ViewType;
    fn from_view(view: Self::ViewType) -> Self;
}

impl<T: View> View for Box<T> {
    type ViewType = Box<T::ViewType>;

    fn to_view(&self) -> Self::ViewType {
        Box::new((**self).to_view())
    }

    fn from_view(view: Self::ViewType) -> Self {
        Self::new(T::from_view(*view))
    }
}

impl<T: View> View for Option<T> {
    type ViewType = Option<T::ViewType>;

    fn to_view(&self) -> Self::ViewType {
        self.as_ref().map(View::to_view)
    }

    fn from_view(view: Self::ViewType) -> Self {
        view.map(T::from_view)
    }
}

impl<T: View> View for Vec<T> {
    type ViewType = Vec<T::ViewType>;

    fn to_view(&self) -> Self::ViewType {
        self.iter().map(View::to_view).collect()
    }

    fn from_view(view: Self::ViewType) -> Self {
        view.into_iter().map(T::from_view).collect()
    }
}

impl View for String {
    type ViewType = Self;

    fn to_view(&self) -> Self::ViewType {
        self.clone()
    }

    fn from_view(view: Self::ViewType) -> Self {
        view
    }
}

// impl_type_view
#[macro_export]
macro_rules! impl_type_view {
    ($($type:ty),*) => {
        $(
            impl View for $type {
                type ViewType = $type;

                fn to_view(&self) -> Self::ViewType {
                    *self
                }

                fn from_view(view: Self::ViewType) -> Self {
                    view
                }
            }
        )*
    };
}

impl_type_view!(bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

///
/// CreateView
///

pub trait CreateView {
    type CreateType: Default;
}

///
/// EditView
///

pub trait EditView {
    type EditType: Default;

    /// Merge `view` into `self`, skipping `None` fields.
    fn merge(&mut self, view: Self::EditType);
}

///
/// FilterView
///

pub trait FilterView {
    type FilterType: Default;

    /// Converts the filter view into a `FilterExpr` suitable for execution.
    fn into_expr(view: Self::FilterType) -> Option<FilterExpr>;
}

/// ------------------------
/// OTHER TRAITS
/// ------------------------

///
/// FieldValues
///

pub trait FieldValues {
    fn get_value(&self, field: &str) -> Option<Value>;
}

///
/// FieldValue
///
/// A trait that defines how a value is wrapped for WHERE queries,
/// filtering, or comparison.
///

pub trait FieldValue {
    // returns an opaque sentinel type by default
    fn to_value(&self) -> Value {
        Value::Unsupported
    }
}

impl FieldValue for &str {
    fn to_value(&self) -> Value {
        Value::Text((*self).to_string())
    }
}

impl FieldValue for String {
    fn to_value(&self) -> Value {
        Value::Text(self.clone())
    }
}

impl<T: FieldValue + Clone> FieldValue for &T {
    fn to_value(&self) -> Value {
        (*self).clone().to_value()
    }
}

// impl_field_value_as
#[macro_export]
macro_rules! impl_field_value_as {
    ( $( $type:ty => $variant:ident ),* $(,)? ) => {
        $(
            impl FieldValue for $type {
                fn to_value(&self) -> Value {
                    Value::$variant((*self).into())
                }
            }
        )*
    };
}

impl_field_value_as!(
    i8 => Int,
    i16 => Int,
    i32 => Int,
    i64 => Int,
    u8 => Uint,
    u16 => Uint,
    u32 => Uint,
    u64 => Uint,
    bool => Bool,
);

///
/// Inner
/// for Newtypes to get the innermost value
///
/// DO NOT REMOVE - its been added and removed twice already, NumCast
/// is a pain to use and won't work for half our types
///

pub trait Inner<T> {
    fn inner(&self) -> &T;
    fn into_inner(self) -> T;
}

// impl_inner
#[macro_export]
macro_rules! impl_inner {
    ($($type:ty),*) => {
        $(
            impl Inner<$type> for $type {
                fn inner(&self) -> &$type {
                    &self
                }
                fn into_inner(self) -> $type {
                    self
                }
            }
        )*
    };
}

impl_inner!(
    bool, f32, f64, i8, i16, i32, i64, i128, String, u8, u16, u32, u64, u128
);

///
/// Path
///
/// any node created via a macro has a Path
/// ie. design::game::rarity::Rarity
///

pub trait Path {
    const PATH: &'static str;
}

///
/// Sanitizer
/// transforms a value into a sanitized version
///

pub trait Sanitizer<T: ?Sized> {
    /// Takes ownership of `value` and returns a sanitized version.
    fn sanitize(&self, value: T) -> T;
}

///
/// Validator
/// allows a node to validate different types of primitives
///

pub trait Validator<T: ?Sized> {
    fn validate(&self, value: &T) -> Result<(), String>;
}
