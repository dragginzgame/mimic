// re-exports of other traits
// for the standard traits::X pattern
pub use icu::ic::structures::storable::Storable;
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
    common::error::ErrorTree,
    core::{
        Key, Value, ValueMap,
        types::{Decimal, Ulid},
        visit::Visitor,
    },
    db::{executor::SaveExecutor, query::SortExpr},
    schema::node::EntityIndex,
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
        impl $trait for u8 {}
        impl $trait for u16 {}
        impl $trait for u32 {}
        impl $trait for u64 {}
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
/// any data type
///

pub trait TypeKind:
    Kind + Clone + Default + Serialize + DeserializeOwned + Visitable + PartialEq + TypeView
{
}

impl<T> TypeKind for T where
    T: Kind + Clone + Default + Serialize + DeserializeOwned + Visitable + PartialEq + TypeView
{
}

///
/// EntityKind
///

pub trait EntityKind: TypeKind + EntitySearch + EntitySort {
    type PrimaryKey: Copy + Clone;

    const STORE: &'static str;
    const PRIMARY_KEY: &'static str;
    const INDEXES: &'static [EntityIndex];

    fn primary_key(&self) -> Self::PrimaryKey;
    fn key(&self) -> Key;

    fn values(&self) -> ValueMap;
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

    // key
    #[must_use]
    fn key(&self) -> Key {
        self.ulid().into()
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

pub trait FieldKind: FieldValue + FieldSearchable + FieldSortable {}
impl<T: FieldValue + FieldSearchable + FieldSortable> FieldKind for T {}

///
/// ANY KIND TRAITS
///

///
/// TypeView
///

pub trait TypeView {
    type View;

    fn to_view(&self) -> Self::View;
    fn from_view(view: Self::View) -> Self;
}

pub type View<T> = <T as TypeView>::View;

impl<T: TypeView> TypeView for Box<T> {
    type View = Box<T::View>;

    fn to_view(&self) -> Self::View {
        Box::new((**self).to_view())
    }

    fn from_view(view: Self::View) -> Self {
        Self::new(T::from_view(*view))
    }
}

impl<T: TypeView> TypeView for Option<T> {
    type View = Option<T::View>;

    fn to_view(&self) -> Self::View {
        self.as_ref().map(TypeView::to_view)
    }

    fn from_view(view: Self::View) -> Self {
        view.map(T::from_view)
    }
}

impl<T: TypeView> TypeView for Vec<T> {
    type View = Vec<T::View>;

    fn to_view(&self) -> Self::View {
        self.iter().map(TypeView::to_view).collect()
    }

    fn from_view(view: Self::View) -> Self {
        view.into_iter().map(T::from_view).collect()
    }
}

impl TypeView for String {
    type View = Self;

    fn to_view(&self) -> Self::View {
        self.clone()
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

// impl_primitive_type_view
#[macro_export]
macro_rules! impl_primitive_type_view {
    ($($type:ty),*) => {
        $(
            impl TypeView for $type {
                type View = $type;

                fn to_view(&self) -> Self::View {
                    *self
                }

                fn from_view(view: Self::View) -> Self {
                    view
                }
            }
        )*
    };
}

impl_primitive_type_view!(bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

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

type EntitySortFn<E> = dyn Fn(&E, &E) -> Ordering;

pub trait EntitySort {
    fn sort(expr: &SortExpr) -> Box<EntitySortFn<Self>>
    where
        Self: Sized;
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

pub trait ValidatorDecimal {
    fn validate(&self, _: &Decimal) -> Result<(), String> {
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
/// FieldSearchable
///

pub trait FieldSearchable {
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

impl<T: FieldSearchable> FieldSearchable for Option<T> {
    fn contains_text(&self, text: &str) -> bool {
        self.as_ref().is_some_and(|v| v.contains_text(text))
    }
}

impl<T: FieldSearchable> FieldSearchable for Vec<T> {
    fn contains_text(&self, text: &str) -> bool {
        self.iter().any(|v| v.contains_text(text))
    }
}

// impl_primitive_field_search
#[macro_export]
macro_rules! impl_primitive_field_search {
    ($($type:ty),*) => {
        $(
            impl FieldSearchable for $type {
                fn to_searchable_string(&self) -> Option<String> {
                    Some(self.to_string())
                }
            }
        )*
    };
}

impl_primitive_field_search!(bool, i8, i16, i32, i64, String, u8, u16, u32, u64, f32, f64);

///
/// FieldSortable
///
/// wrapper around the Ord/PartialOrd traits so that we can extend it to
/// more ORM types
///

pub trait FieldSortable {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl FieldSortable for f32 {}
impl FieldSortable for f64 {}

// impl_primitive_field_orderable
#[macro_export]
macro_rules! impl_primitive_field_orderable {
    ($($type:ty),*) => {
        $(
            impl FieldSortable for $type {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    std::cmp::Ord::cmp(self, other)
                }
            }
        )*
    };
}

impl_primitive_field_orderable!(bool, i8, i16, i32, i64, String, u8, u16, u32, u64);

impl<T: FieldSortable> FieldSortable for Box<T> {}

impl<T: FieldSortable> FieldSortable for Option<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(a), Some(b)) => a.cmp(b),
        }
    }
}

///
/// FieldValue
///
/// A trait that defines how a value is wrapped for WHERE queries,
/// filtering, or comparison.
///

pub trait FieldValue {
    fn to_value(&self) -> Value {
        Value::Unsupported
    }
}

impl FieldValue for String {
    fn to_value(&self) -> Value {
        Value::Text(self.clone())
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
    f32 => Float,
    f64 => Float,
    i8 => Int,
    i16 => Int,
    i32 => Int,
    i64 => Int,
    u8 => Nat,
    u16 => Nat,
    u32 => Nat,
    u64 => Nat,
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

impl_primitive_inner!(bool, f32, f64, i8, i16, i32, i64, u8, u16, u32, u64);

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

impl<T: ValidateAuto> ValidateAuto for Option<T> {}
impl<T: ValidateAuto> ValidateAuto for Vec<T> {}
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

impl<T: ValidateCustom> ValidateCustom for Option<T> {}
impl<T: ValidateCustom> ValidateCustom for Vec<T> {}
impl<T: ValidateCustom> ValidateCustom for Box<T> {}

impl_primitive!(ValidateCustom);

///
/// Visitable
///

pub trait Visitable: Validate {
    fn drive(&self, _: &mut dyn Visitor) {}
    fn drive_mut(&mut self, _: &mut dyn Visitor) {}
}

impl<T: Visitable> Visitable for Option<T> {
    fn drive(&self, visitor: &mut dyn crate::core::visit::Visitor) {
        if let Some(value) = self {
            crate::core::visit::perform_visit(visitor, value, "");
        }
    }
}

impl<T: Visitable> Visitable for Vec<T> {
    fn drive(&self, visitor: &mut dyn crate::core::visit::Visitor) {
        for (i, value) in self.iter().enumerate() {
            let key = i.to_string();
            crate::core::visit::perform_visit(visitor, value, &key);
        }
    }
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
