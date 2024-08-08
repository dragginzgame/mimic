// re-exports of other traits
// for the standard traits::X pattern
pub use candid::CandidType;
pub use ic::structures::storable::Storable;
pub use num_traits::{FromPrimitive as NumFromPrimitive, NumCast, ToPrimitive as NumToPrimitive};
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use std::{
    cmp::Ordering,
    convert::{AsRef, From},
    default::Default,
    fmt::{Debug, Display},
    hash::Hash,
    iter::IntoIterator,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::{types::SortDirection, visit::Visitor, Error};
use ::types::ErrorVec;

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
/// Filterable
///
/// a trait that allows you to optionally allow a type to
/// be filtered based on the string representation
///

pub trait Filterable {
    // as_text
    // implement this if you want a type to be filtered as text
    fn as_text(&self) -> Option<String> {
        None
    }

    // contains_text
    fn contains_text(&self, text: &str) -> bool {
        self.as_text()
            .map_or(false, |s| s.to_lowercase().contains(&text.to_lowercase()))
    }
}

impl Filterable for String {
    fn as_text(&self) -> Option<String> {
        Some(self.to_string())
    }
}

// impl_primitive_filter
#[macro_export]
macro_rules! impl_primitive_filterable {
    ($($type:ty),*) => {
        $(
            impl Filterable for $type {}
        )*
    };
}

impl_primitive_filterable!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, bool);

///
/// Inner
///

pub trait Inner<T> {
    fn inner(&self) -> &T;
}

// impl_primitive_inner
#[macro_export]
macro_rules! impl_primitive_inner {
    ($($type:ty),*) => {
        $(
            impl Inner<$type> for $type {
                fn inner(&self) -> &$type {
                    &self
                }
            }
        )*
    };
}

impl_primitive_inner!(bool, f32, f64, i8, i16, i32, i64, i128, String, u8, u16, u32, u64, u128);

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

impl_primitive_order!(bool, i8, i16, i32, i64, i128, String, u8, u16, u32, u64, u128);

impl<T: Orderable> Orderable for Option<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            // Both are None, they are equal
            (None, None) => std::cmp::Ordering::Equal,

            // Any None is less than Some
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,

            // If both are Some, compare the inner values
            (Some(ref a), Some(ref b)) => a.cmp(b),
        }
    }
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

macro_rules! impl_primitive_list {
    ($($t:ty => $name:expr),* $(,)?) => {
        $(
            impl Path for $t {
                const IDENT: &'static str = $name;
                const PATH: &'static str = concat!("mimic_base::types::", $name);
            }
        )*
    };
}

impl_primitive_list!(
    i8 => "I8", i16 => "I16", i32 => "I32", i64 => "I64", i128 => "I128",
    u8 => "U8", u16 => "U16", u32 => "U32", u64 => "U64", u128 => "U128",
    f32 => "F32", f64 => "F64", bool => "Bool", String => "String"
);

///
/// Sanitize
///

pub trait Sanitize: SanitizeAuto {
    fn sanitize(&mut self) {
        self.sanitize_manual();
        self.sanitize_auto();
    }

    fn sanitize_manual(&mut self) {}
}

impl_primitive!(Sanitize);

///
/// SanitizeAuto
///

pub trait SanitizeAuto {
    fn sanitize_auto(&mut self) {}
}

impl_primitive!(SanitizeAuto);

///
/// Validate
///
/// The default behaviour is Ok() so no errors unless
/// this method is overridden
///

pub trait Validate: ValidateAuto {
    fn validate(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();
        errs.merge(self.validate_manual());
        errs.merge(self.validate_auto());

        errs.result()
    }

    fn validate_manual(&self) -> Result<(), ErrorVec> {
        Ok(())
    }
}

impl_primitive!(Validate);

///
/// ValidateAuto
///
/// this is for extra derived methods, such as checking invalid
/// variants of an enum
///

pub trait ValidateAuto {
    fn validate_auto(&self) -> Result<(), ErrorVec> {
        Ok(())
    }
}

impl_primitive!(ValidateAuto);

///
/// Visitable
///

pub trait Visitable: Validate + Sanitize {
    fn drive(&self, _: &mut dyn Visitor) {}
    fn drive_mut(&mut self, _: &mut dyn Visitor) {}
}

impl_primitive!(Visitable);

///
/// OPTIONAL
///

///
/// PrimaryKey
///

pub trait PrimaryKey: FromStr {
    // on_create
    // this is the value that the primary key would be set to
    // on a store CREATE call
    #[must_use]
    fn on_create(&self) -> Self;

    // format_key
    // how is this type formatted within a sort key string
    fn format(&self) -> String;
}

macro_rules! impl_primary_key_for_ints {
    ($($t:ty, $ut:ty, $len:expr),* $(,)?) => {
        $(
            impl PrimaryKey for $t {
                fn on_create(&self) -> Self {
                    *self
                }

                #[allow(clippy::cast_sign_loss)]
                fn format(&self) -> String {
                    if *self < 0 {
                        let inverted = <$ut>::MAX - self.wrapping_abs() as $ut;
                        format!("-{:0>width$}", inverted, width = $len - 1)
                    } else {
                        format!("{:0>width$}", *self as $ut, width = $len)
                    }
                }
            }
        )*
    };
}

macro_rules! impl_primary_key_for_uints {
    ($($t:ty, $len:expr),* $(,)?) => {
        $(
            impl PrimaryKey for $t {
                fn on_create(&self) -> Self {
                    *self
                }

                fn format(&self) -> String {
                    format!("{:0>width$}", self, width = $len)
                }
            }
        )*
    };
}

#[rustfmt::skip]
impl_primary_key_for_ints!(
    i8, u8, 4,
    i16, u16, 6,
    i32, u32, 11,
    i64, u64, 21,
    i128, u128, 41
);

#[rustfmt::skip]
impl_primary_key_for_uints!(
    u8, 3,
    u16, 5,
    u32, 10,
    u64, 20,
    u128, 40
);

///
/// NODE TRAITS
///

///
/// EntityDerive
///

pub trait EntityDerive: Serialize + DeserializeOwned {
    type Entity: Entity;

    // assemble
    fn assemble(entity: Self::Entity) -> Result<Self, Error>
    where
        Self: Sized;
}

///
/// Entity
///

pub trait Entity:
    EntityDynamic + Clone + Path + FieldSort + FieldFilter + Serialize + DeserializeOwned
{
    // composite_key
    // allows you to construct a key by passing in values
    fn composite_key(_keys: &[String]) -> Result<Vec<String>, Error>;
}

///
/// EntityDynamic
/// everything the Entity needs to interact with the Store dynamically
///

pub trait EntityDynamic: Debug + Visitable {
    // on_create
    // modifies the entity's record in-place before saving it to the database
    fn on_create(&mut self) {}

    // composite_key_dyn
    // returns the record's composite key (parent keys + primary key) as a Vec<String>
    fn composite_key_dyn(&self) -> Vec<String>;

    // path_dyn
    fn path_dyn(&self) -> String;

    // serialize_dyn
    fn serialize_dyn(&self) -> Result<Vec<u8>, Error>;
}

///
/// EntityFixture
/// an enum that can generate fixture data for an Entity
///

pub trait EntityFixture: Into<&'static str> {
    #[must_use]
    fn fixtures() -> Vec<Box<dyn EntityDynamic>>;

    // boxed
    // create a boxed trait object for fixtures
    fn boxed<E: EntityDynamic + 'static>(entity: E) -> Box<dyn EntityDynamic> {
        Box::new(entity) as Box<dyn EntityDynamic>
    }
}

///
/// EnumHash
///
/// allows an enum's variants to always anchor to the same ID so they
/// can have a permanent relation to the backend code and services
///
/// don't mix this with Fixtures as otherwise we could have a situation where
/// we can't delete the Fixture data because it would destroy the Key logic
///

pub trait EnumHash: Sized {
    fn to_hash(&self) -> u64;
    fn try_from_hash(n: u64) -> Result<Self, Error>;
}

///
/// FieldFilter
///
/// allows anything with a collection of fields to be filtered
/// None means search all fields
///

pub trait FieldFilter {
    fn list_fields(&self) -> &'static [&'static str];
    fn filter_field(&self, field: &str, text: &str) -> bool;

    // filter_fields
    // AND so we want to return if any specified field doesn't match
    fn filter_fields(&self, fields: Vec<(String, String)>) -> bool {
        for (field, text) in fields {
            if !self.filter_field(&field, &text) {
                return false;
            }
        }

        true
    }

    // filter_all
    // true if any field matches
    fn filter_all(&self, text: &str) -> bool {
        for field in self.list_fields() {
            if self.filter_field(field, text) {
                return true;
            }
        }

        false
    }
}

///
/// FieldSort
///
/// allows anything with a collection of fields to be sorted
///

type FieldSortFn<E> = dyn Fn(&E, &E) -> ::std::cmp::Ordering;

pub trait FieldSort {
    fn default_order() -> Vec<(String, SortDirection)>;

    // sort
    // pass in a blank slice to sort by the default
    #[must_use]
    fn sort(order: &[(String, SortDirection)]) -> Box<FieldSortFn<Self>> {
        let order = if order.is_empty() {
            &Self::default_order()
        } else {
            order
        };

        Self::generate_sorter(order)
    }

    fn generate_sorter(order: &[(String, SortDirection)]) -> Box<FieldSortFn<Self>>;
}
