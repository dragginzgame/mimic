// re-exports of other traits
// for the standard traits::X pattern
pub use icu::ic::{candid::CandidType, structures::storable::Storable};
pub use num_traits::{FromPrimitive as NumFromPrimitive, NumCast, ToPrimitive as NumToPrimitive};
pub use serde::{Deserialize, Serialize, de::DeserializeOwned};
pub use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    convert::{AsRef, From, Into},
    default::Default,
    fmt::{Debug, Display},
    hash::Hash,
    iter::IntoIterator,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::{
    db::{
        executor::SaveExecutor,
        query::SortDirection,
        store::{DataKey, IndexKey},
    },
    error::ErrorTree,
    ops::{
        types::{IndexValue, Value},
        visit::Visitor,
    },
    schema::node::EntityIndex,
    types::{EntityKey, Ulid},
};

///
/// MACROS
///

// impl_primitive
#[macro_export]
macro_rules! impl_primitive {
    ($trait:ident) => {
        impl $trait for i8 {}
        impl $trait for i16 {}
        impl $trait for i32 {}
        impl $trait for i64 {}
        impl $trait for i128 {}
        impl $trait for u8 {}
        impl $trait for u16 {}
        impl $trait for u32 {}
        impl $trait for u64 {}
        impl $trait for u128 {}
        impl $trait for f32 {}
        impl $trait for f64 {}
        impl $trait for bool {}
        impl $trait for String {}
    };
}

///
/// KIND TRAITS
/// the Schema uses the term "Node" but when they're built it's "Kind"
///

///
/// Kind
///

pub trait Kind: Path {}

impl<T> Kind for T where T: Path {}

///
/// TypeKind
/// a Meta that can act as a data type
///

pub trait TypeKind:
    Kind + CandidType + Clone + Default + Serialize + DeserializeOwned + Visitable
{
}

impl<T> TypeKind for T where
    T: Kind + CandidType + Clone + Default + Serialize + DeserializeOwned + Visitable
{
}

///
/// EntityKind
///

pub trait EntityKind: TypeKind + EntitySearch + EntitySort + PartialEq {
    const STORE: &'static str;
    const INDEXES: &'static [EntityIndex];

    // values
    fn values(&self, fields: &[&str]) -> Vec<Value>;
    fn index_values(&self, fields: &[&str]) -> Vec<IndexValue>;

    // entity_key
    // returns the current entity key, ie [V::Text("00AX5"), V::Nat8(1)]
    fn entity_key(&self) -> EntityKey;

    // data_key
    // builds the data key using the current entity key
    fn data_key(&self) -> DataKey {
        Self::build_data_key(&self.entity_key())
    }
    fn build_data_key(values: &[IndexValue]) -> DataKey;

    // build_index_key
    // returns the current index key for specific fields, ie [V::Nat32(0), V::Nat32(16)]
    fn index_key(&self, fields: &[&str]) -> Option<IndexKey> {
        IndexKey::build(Self::PATH, fields, &self.index_values(fields))
    }
}

///
/// EntityIdKind
///

pub trait EntityIdKind: Kind + std::fmt::Debug {
    #[must_use]
    fn ulid(&self) -> Ulid {
        let digest = format!("{}-{:?}", Self::PATH, self);

        Ulid::from_string_digest(&digest)
    }

    // relation
    #[must_use]
    fn entity_key(&self) -> EntityKey {
        let iv = self
            .ulid()
            .to_index_value()
            .expect("entityid has an index value");

        EntityKey(vec![iv])
    }
}

///
/// EnumValueKind
///

pub trait EnumValueKind {
    fn value(&self) -> i32;
}

///
/// FieldKind
///

pub trait FieldKind: FieldValue + FieldSearch {}
impl<T: FieldValue + FieldSearch> FieldKind for T {}

///
/// ANY KIND TRAITS
///

///
/// Path
///
/// any node created via a macro has a Path
/// ie. design::game::rarity::Rarity
///
/// primitives are used unwrapped so we can't declare the impl anywhere else
///

pub trait Path {
    const PATH: &'static str;

    #[must_use]
    fn path() -> String {
        Self::PATH.to_string()
    }
}

///
/// SINGLE KIND TRAITS
///

///
/// EntityFixture
/// an enum that can generate fixture data for an Entity
///

pub trait EntityFixture {
    // fixtures
    // inserts the fixtures to the bd via the SaveExecutor
    fn insert_fixtures(_exec: &mut SaveExecutor) {}
}

///
/// EntitySearch
///

pub trait EntitySearch {
    fn search_field(&self, field: &str, text: &str) -> bool;

    // search_fields
    // AND so we want to return if any specified field doesn't match
    fn search_fields(&self, fields: &[(String, String)]) -> bool {
        for (field, text) in fields {
            if !self.search_field(field, text) {
                return false;
            }
        }

        true
    }
}

///
/// EntitySort
/// allows anything with a collection of fields to be sorted
///

type EntitySortFn<E> = dyn Fn(&E, &E) -> ::std::cmp::Ordering;

pub trait EntitySort {
    fn sort(order: &[(String, SortDirection)]) -> Box<EntitySortFn<Self>>;
}

///
/// Validator
/// allows a node to validate different types of primitives
///

pub trait ValidatorBytes {
    fn validate(&self, _: &[u8]) -> Result<(), String> {
        Ok(())
    }
}

pub trait ValidatorNumber {
    fn validate<T>(&self, _: &T) -> Result<(), String>
    where
        T: Copy + NumCast,
    {
        Ok(())
    }
}

pub trait ValidatorString {
    fn validate<S>(&self, _: S) -> Result<(), String>
    where
        S: AsRef<str>,
    {
        Ok(())
    }
}

///
/// TYPE TRAITS
///

///
/// FieldIndexValue
/// optional, a field can be turned into an IndexValue
///

pub trait FieldIndexValue {
    fn to_index_value(&self) -> Option<IndexValue> {
        None
    }
}

impl FieldIndexValue for String {
    fn to_index_value(&self) -> Option<IndexValue> {
        Some(IndexValue::Text(self.clone()))
    }
}
impl FieldIndexValue for bool {}
impl FieldIndexValue for f32 {}
impl FieldIndexValue for f64 {}

// impl_field_value_as
#[macro_export]
macro_rules! impl_field_index_value_as {
    ( $( $type:ty => $variant:ident ),* $(,)? ) => {
        $(
            impl FieldIndexValue for $type {
                fn to_index_value(&self) -> Option<IndexValue> {
                    Some(IndexValue::$variant(*self as _))
                }
            }
        )*
    };
}

impl_field_index_value_as!(
    i8 => Int,
    i16 => Int,
    i32 => Int,
    i64 => Int,
    i128 => Int,
    u8 => Nat,
    u16 => Nat,
    u32 => Nat,
    u64 => Nat,
    u128 => Nat,
);

///
/// FieldOrderable
///
/// wrapper around the Ord/PartialOrd traits so that we can extend it to
/// more ORM types
///

pub trait FieldOrderable {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl FieldOrderable for f32 {}
impl FieldOrderable for f64 {}
impl<T: FieldOrderable> FieldOrderable for Box<T> {}

// impl_primitive_field_orderable
#[macro_export]
macro_rules! impl_primitive_field_orderable {
    ($($type:ty),*) => {
        $(
            impl FieldOrderable for $type {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    std::cmp::Ord::cmp(self, other)
                }
            }
        )*
    };
}

impl_primitive_field_orderable!(
    bool, i8, i16, i32, i64, i128, String, u8, u16, u32, u64, u128
);

impl<T: FieldOrderable> FieldOrderable for Option<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            // Both are None, they are equal
            (None, None) => std::cmp::Ordering::Equal,

            // Any None is less than Some
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,

            // If both are Some, compare the inner values
            (Some(a), Some(b)) => a.cmp(b),
        }
    }
}

///
/// FieldSearch
///

pub trait FieldSearch {
    /// Returns a canonical string form of the value, if available
    /// if None is returned it means that the type is just not suitable for searching
    fn to_searchable_string(&self) -> Option<String> {
        None
    }

    /// Case-insensitive containment check using the query value.
    fn contains_text(&self, s: &str) -> bool {
        self.to_searchable_string()
            .is_some_and(|val| val.to_lowercase().contains(&s.to_lowercase()))
    }
}

// impl_primitive_field_search
#[macro_export]
macro_rules! impl_primitive_field_search {
    ($($type:ty),*) => {
        $(
            impl FieldSearch for $type {
                fn to_searchable_string(&self) -> Option<String> {
                    Some(self.to_string())
                }
            }
        )*
    };
}

impl_primitive_field_search!(
    bool, i8, i16, i32, i64, i128, String, u8, u16, u32, u64, u128, f32, f64
);

///
/// FieldValue
///
/// A trait that defines how a value is wrapped for WHERE queries,
/// filtering, or comparison.
///

pub trait FieldValue {
    fn to_value(&self) -> Option<Value> {
        None
    }
}

impl FieldValue for String {
    fn to_value(&self) -> Option<Value> {
        Some(Value::Text(self.clone()))
    }
}

// impl_field_value_as
#[macro_export]
macro_rules! impl_field_value_as {
    ( $( $type:ty => $variant:ident ),* $(,)? ) => {
        $(
            impl FieldValue for $type {
                fn to_value(&self) -> Option<Value> {
                    Some(Value::$variant(*self as _))
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
    i128 => Int,
    u8 => Nat,
    u16 => Nat,
    u32 => Nat,
    u64 => Nat,
    u128 => Nat,
    f32 => Float,
    f64 => Float,
    bool => Bool,
);

///
/// Inner
/// a trait for Newtypes to recurse downwards to find the innermost value
///

pub trait Inner {
    type Primitive;

    fn inner(&self) -> Self::Primitive;
    fn into_inner(self) -> Self::Primitive;
}

impl Inner for String {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        self.clone()
    }

    fn into_inner(self) -> Self::Primitive {
        self
    }
}

// impl_primitive_inner
#[macro_export]
macro_rules! impl_primitive_inner {
    ($($type:ty),*) => {
        $(
            impl Inner for $type {
                type Primitive = Self;

                fn inner(&self) -> Self::Primitive {
                    *self
                }
                fn into_inner(self) -> Self::Primitive {
                    self
                }
            }
        )*
    };
}

impl_primitive_inner!(
    bool, f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
);

///
/// Validate
///

pub trait Validate: ValidateAuto + ValidateCustom {
    fn validate(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();

        if let Err(e) = self.validate_self() {
            errs.merge(e);
        }
        if let Err(e) = self.validate_children() {
            errs.merge(e);
        }
        if let Err(e) = self.validate_custom() {
            errs.merge(e);
        }

        errs.result()
    }
}

impl<T> Validate for T where T: ValidateAuto + ValidateCustom {}

///
/// ValidateAuto
///
/// derived code that is used to generate the validation rules for a type and
/// its children, via schema validation rules
///

pub trait ValidateAuto {
    fn validate_self(&self) -> Result<(), ErrorTree> {
        Ok(())
    }

    fn validate_children(&self) -> Result<(), ErrorTree> {
        Ok(())
    }
}

impl<T: ValidateAuto> ValidateAuto for Box<T> {}
impl_primitive!(ValidateAuto);

///
/// ValidateCustom
///
/// custom validation behaviour that can be added to any type
///

pub trait ValidateCustom {
    fn validate_custom(&self) -> Result<(), ErrorTree> {
        Ok(())
    }
}

impl<T: ValidateCustom> ValidateCustom for Box<T> {}
impl_primitive!(ValidateCustom);

///
/// Visitable
///

pub trait Visitable: Validate {
    fn drive(&self, _: &mut dyn Visitor) {}
    fn drive_mut(&mut self, _: &mut dyn Visitor) {}
}

impl<T: Visitable> Visitable for Box<T> {
    fn drive(&self, visitor: &mut dyn Visitor) {
        (**self).drive(visitor);
    }

    fn drive_mut(&mut self, visitor: &mut dyn Visitor) {
        (**self).drive_mut(visitor);
    }
}

impl_primitive!(Visitable);
