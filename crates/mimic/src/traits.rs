// re-exports of other traits
// for the standard traits::X pattern
pub use candid::CandidType;
pub use ic_stable_structures::storable::Storable;
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
    data::{
        executor::SaveExecutor,
        query::{SaveMode, SaveQueryTyped},
    },
    schema::types::SortDirection,
    types::{
        ErrorTree,
        prim::{Key, Ulid},
    },
    visit::Visitor,
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
    Kind + CandidType + Clone + Default + FormatSortKey + Serialize + DeserializeOwned
{
}

impl<T> TypeKind for T where
    T: Kind + CandidType + Clone + Default + FormatSortKey + Serialize + DeserializeOwned
{
}

///
/// EntityKind
///

pub trait EntityKind:
    TypeKind + EntityFixture + EntitySearch + EntitySort + PartialEq + Visitable
{
    fn key_values(&self) -> HashMap<String, Option<String>>;
}

///
/// EntityIdKind
///

pub trait EntityIdKind: Kind + Display {
    #[must_use]
    fn ulid(&self) -> Ulid {
        let digest = format!("{}-{}", Self::path(), self);

        Ulid::from_string_digest(&digest)
    }

    #[must_use]
    fn key(&self) -> Key {
        Key::from(vec![self.ulid()])
    }
}

///
/// EnumValueKind
///

pub trait EnumValueKind {
    fn value(&self) -> i32;
}

///
/// TRAITS
///

///
/// EntityFixture
/// an enum that can generate fixture data for an Entity
///

pub trait EntityFixture {
    // fixtures
    // inserts the fixtures to the bd via the SaveExecutor
    fn insert_fixtures(_exec: &mut SaveExecutor) {}

    // add
    // helper function to execute query
    fn add<E: EntityKind>(exec: &mut SaveExecutor, entity: E) {
        let q = SaveQueryTyped::new(SaveMode::Replace, entity);

        exec.execute::<E>(q).unwrap();
    }
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
/// Path
///
/// any node created via a macro has a Path
/// ie. design::game::rarity::Rarity
///
/// primitives are used unwrapped so we can't declare the impl anywhere else
///

pub trait Path {
    const IDENT: &'static str;
    const PATH: &'static str;

    #[must_use]
    fn ident() -> String {
        Self::IDENT.to_string()
    }

    #[must_use]
    fn path() -> String {
        Self::PATH.to_string()
    }
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
        T: Copy + Display + NumCast,
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
/// FormatSortKey
/// a type that can be formatted as a Sort Key
///

pub trait FormatSortKey {
    fn format_sort_key(&self) -> Option<String>;
}

impl FormatSortKey for String {
    fn format_sort_key(&self) -> Option<String> {
        Some(self.to_string())
    }
}

macro_rules! impl_format_sort_key_ints {
    ($($t:ty, $ut:ty, $len:expr),* $(,)?) => {
        $(
            impl FormatSortKey for $t {
                #[allow(clippy::cast_sign_loss)]
                fn format_sort_key(&self) -> Option<String> {
                    if *self < 0 {
                        let inverted = <$ut>::MAX - self.wrapping_abs() as $ut;
                        Some(format!("-{:0>width$}", inverted, width = $len - 1))
                    } else {
                        Some(format!("{:0>width$}", *self as $ut, width = $len))
                    }
                }
            }
        )*
    };
}

macro_rules! impl_format_sort_key_uints {
    ($($t:ty, $len:expr),* $(,)?) => {
        $(
            impl FormatSortKey for $t {
                fn format_sort_key(&self) -> Option<String> {
                    Some(format!("{:0>width$}", self, width = $len))
                }
            }
        )*
    };
}

macro_rules! impl_format_sort_key_none {
    ($($t:ty),* $(,)?) => {
        $(
            impl FormatSortKey for $t {
                fn format_sort_key(&self) -> Option<String> {
                    None
                }
            }
        )*
    };
}

#[rustfmt::skip]
impl_format_sort_key_ints!(
    i8, u8, 4,
    i16, u16, 6,
    i32, u32, 11,
    i64, u64, 21,
    i128, u128, 41
);

#[rustfmt::skip]
impl_format_sort_key_uints!(
    u8, 3,
    u16, 5,
    u32, 10,
    u64, 20,
    u128, 40
);

impl_format_sort_key_none!(bool, f32, f64);

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
/// Orderable
///
/// wrapper around the Ord/PartialOrd traits so that we can extend it to
/// more ORM types
///

pub trait Orderable {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl Orderable for f32 {}
impl Orderable for f64 {}
impl<T: Orderable> Orderable for Box<T> {}

// impl_primitive_order
#[macro_export]
macro_rules! impl_primitive_order {
    ($($type:ty),*) => {
        $(
            impl Orderable for $type {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    std::cmp::Ord::cmp(self, other)
                }
            }
        )*
    };
}

impl_primitive_order!(
    bool, i8, i16, i32, i64, i128, String, u8, u16, u32, u64, u128
);

impl<T: Orderable> Orderable for Option<T> {
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
/// Searchable
///
/// a trait that allows you to optionally allow a type to
/// be searched based on the string representation
///

pub trait Searchable {
    // as_text
    // implement this if you want a type to be searched as text
    fn as_text(&self) -> Option<String> {
        None
    }

    // contains_text
    fn contains_text(&self, text: &str) -> bool {
        self.as_text()
            .is_some_and(|s| s.to_lowercase().contains(&text.to_lowercase()))
    }
}

impl<T: Searchable> Searchable for Box<T> {}

impl Searchable for String {
    fn as_text(&self) -> Option<String> {
        Some(self.to_string())
    }
}

// impl_primitive_searchable
#[macro_export]
macro_rules! impl_primitive_searchable {
    ($($type:ty),*) => {
        $(
            impl Searchable for $type {}
        )*
    };
}

impl_primitive_searchable!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, bool
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
