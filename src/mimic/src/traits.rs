// re-exports of other traits
// for the standard traits::X pattern
pub use candid::CandidType;
pub use ic_stable_structures::storable::Storable;
pub use num_traits::{FromPrimitive as NumFromPrimitive, NumCast, ToPrimitive as NumToPrimitive};
pub use serde::{Deserialize, Serialize, de::DeserializeOwned};
pub use std::{
    cmp::Ordering,
    collections::HashSet,
    convert::{AsRef, From, Into},
    default::Default,
    fmt::{Debug, Display},
    hash::Hash,
    iter::IntoIterator,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::{
    SerializeError,
    schema::types::SortDirection,
    types::{
        ErrorTree, FixtureList,
        prim::{Relation, Ulid},
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
/// ANY NODE TRAITS
///

///
/// Node
///

pub trait Node: Path {}

impl<T> Node for T where T: Path {}

///
/// NodeDyn
///

pub trait NodeDyn {
    // path_dyn
    // as every node needs path, this makes creating dynamic traits easier
    fn path_dyn(&self) -> String;
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
    i8 => "Int8", i16 => "Int16", i32 => "Int32", i64 => "Int64", i128 => "Int128",
    u8 => "Nat8", u16 => "Nat16", u32 => "Nat32", u64 => "Nat64", u128 => "Nat128",
    f32 => "Float32", f64 => "Float64", bool => "Bool", String => "Text"
);

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
/// (for any trait that works on a Type)
///

///
/// Type
/// a Node that can act as a data type
///

pub trait Type: Node + CandidType + Clone + Default + Serialize + DeserializeOwned {}

impl<T> Type for T where T: Node + CandidType + Clone + Default + Serialize + DeserializeOwned {}

///
/// TypeDyn
/// just to keep things symmetrical, not actually used yet other than
/// making sure all types have Debug
///

pub trait TypeDyn: NodeDyn + Debug {}

impl<T> TypeDyn for T where T: NodeDyn + Debug {}

///
/// Inner
/// a trait for Newtypes to recurse downwards to find the innermost value
///

pub trait Inner<T> {
    fn inner(&self) -> &T;
    fn into_inner(self) -> T;
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
                fn into_inner(self) -> $type {
                    self
                }
            }
        )*
    };
}

impl_primitive_inner!(
    bool, f32, f64, i8, i16, i32, i64, i128, String, u8, u16, u32, u64, u128
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

///
/// ENTITY TRAITS
///

///
/// Entity
///

pub trait Entity: Type + EntityFixture + EntityDyn + EntitySearch + EntitySort {
    const STORE: &'static str;
}

///
/// EntityDyn
/// object-safe methods for entities
///

pub trait EntityDyn: TypeDyn + SerializeDyn + Visitable {
    // id
    // returns the id of the entity (as there can be 0 or 1 fields in
    // the entity's sort key)
    fn id(&self) -> Option<String>;

    // composite_key
    // returns the record's sort key values as a Vec<String>
    fn composite_key(&self) -> Vec<String>;

    // store
    // returns the path of the store
    fn store(&self) -> String;
}

///
/// SerializeDyn
///

pub trait SerializeDyn {
    fn serialize(&self) -> Result<Vec<u8>, SerializeError>;
}

impl<T> SerializeDyn for T
where
    T: Entity,
{
    fn serialize(&self) -> Result<Vec<u8>, SerializeError> {
        mimic::serialize(self)
    }
}

///
/// EntityFixture
/// an enum that can generate fixture data for an Entity
///

pub trait EntityFixture: Sized {
    // fixtures
    // returns a vec of entities that are inserted on canister init
    #[must_use]
    fn fixtures() -> FixtureList {
        FixtureList::new()
    }
}

///
/// EntityId
///

pub trait EntityId: NodeDyn + Display {
    #[must_use]
    fn ulid(&self) -> Ulid {
        let digest = format!("{}-{}", self.path_dyn(), self);
        Ulid::from_string_digest(&digest)
    }

    #[must_use]
    fn relation(&self) -> Relation {
        Relation::from(vec![self.ulid()])
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
///
/// allows anything with a collection of fields to be sorted
///

type EntitySortFn<E> = dyn Fn(&E, &E) -> ::std::cmp::Ordering;

pub trait EntitySort {
    fn sort(order: &[(String, SortDirection)]) -> Box<EntitySortFn<Self>>;
}

///
/// SortKeyValue
/// a type that can be used inside a SortKey's value
///

pub trait SortKeyValue: FromStr + ToString {
    // format
    // how is this type formatted within a sort key string, we may want
    // to overwrite for fixed-width values
    fn format(&self) -> String {
        self.to_string()
    }
}

impl SortKeyValue for String {}

macro_rules! impl_sort_key_for_ints {
    ($($t:ty, $ut:ty, $len:expr),* $(,)?) => {
        $(
            impl SortKeyValue for $t {
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

macro_rules! impl_sort_key_for_uints {
    ($($t:ty, $len:expr),* $(,)?) => {
        $(
            impl SortKeyValue for $t {
                fn format(&self) -> String {
                    format!("{:0>width$}", self, width = $len)
                }
            }
        )*
    };
}

#[rustfmt::skip]
impl_sort_key_for_ints!(
    i8, u8, 4,
    i16, u16, 6,
    i32, u32, 11,
    i64, u64, 21,
    i128, u128, 41
);

#[rustfmt::skip]
impl_sort_key_for_uints!(
    u8, 3,
    u16, 5,
    u32, 10,
    u64, 20,
    u128, 40
);

///
/// OTHER NODE TRAITS
///

///
/// EnumValue
///

pub trait EnumValue {
    fn value(&self) -> i32;
}

///
/// Selector
///

pub trait Selector {
    fn value(&self) -> isize;
}
