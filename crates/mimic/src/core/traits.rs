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
    core::{Key, Value, types::Ulid, visit::Visitor},
    db::Db,
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
/// CanisterKind
///

pub trait CanisterKind: Kind {}

///
/// EntityKind
///

pub trait EntityKind: Kind + TypeKind + FieldValues {
    type Store: StoreKind;
    type Indexes: IndexKindTuple;
    type PrimaryKey: Copy + Clone;

    const PRIMARY_KEY: &'static str;
    const NUM_FIELDS: usize;

    fn key(&self) -> Key;
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

pub trait EnumValueKind: Kind {
    fn value(&self) -> i32;
}

///
/// FieldKind
///

pub trait FieldKind: Kind + FieldValue {}
impl<T: Kind + FieldValue> FieldKind for T {}

///
/// IndexKind
///

pub trait IndexKind: Kind {
    type Store: StoreKind;
    type Entity: EntityKind;

    const FIELDS: &'static [&'static str];
    const UNIQUE: bool;
}

// Trait to represent a compile-time operation on a single index type
pub trait IndexKindFn {
    type Error;

    fn apply<I: IndexKind>(&mut self) -> Result<(), Self::Error>;
}

// Trait implemented for tuples of IndexKind types
pub trait IndexKindTuple {
    const HAS_INDEXES: bool;

    fn for_each<F: IndexKindFn>(f: &mut F) -> Result<(), F::Error>;
}

impl IndexKindTuple for () {
    const HAS_INDEXES: bool = false;

    fn for_each<F: IndexKindFn>(_: &mut F) -> Result<(), F::Error> {
        Ok(())
    }
}

macro_rules! impl_index_kind_tuple {
    ( $( $name:ident ),+ ) => {
        #[allow(unused_parens)]
        impl< $( $name: IndexKind ),+ > IndexKindTuple for ( $( $name ),+ ) {
            const HAS_INDEXES: bool = true;

            fn for_each<F: IndexKindFn>(f: &mut F) -> Result<(), F::Error> {
                $( f.apply::<$name>()?; )+

                Ok(())
            }
        }
    };
}

impl_index_kind_tuple!(I1);
impl_index_kind_tuple!(I1, I2);
impl_index_kind_tuple!(I1, I2, I3);
impl_index_kind_tuple!(I1, I2, I3, I4);

///
/// StoreKind
///

pub trait StoreKind: Kind {
    type Canister: CanisterKind;
}

///
/// GROUPED KIND TRAITS
///

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
/// OTHER TRAITS
///

///
/// EntityFixture
/// an enum that can generate fixture data for an Entity
///

pub trait EntityFixture {
    // fixtures
    // inserts the fixtures to the bd via the SaveExecutor
    fn insert_fixtures(_: Db) {}
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
/// FieldValues
///

pub trait FieldValues {
    fn get_value(&self, field: &str) -> Option<Value>;
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
    const PATH: &'static str;

    #[must_use]
    fn path() -> String {
        Self::PATH.to_string()
    }
}

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
/// this shouldn't be used with primitive types, it's only really for validation
/// rules put in by macros
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
/// Validator
/// allows a node to validate different types of primitives
///

pub trait Validator<T: ?Sized> {
    fn validate(&self, value: &T) -> Result<(), String>;
}

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
            crate::core::visit::perform_visit(visitor, value, None);
        }
    }
}

impl<T: Visitable> Visitable for Vec<T> {
    fn drive(&self, visitor: &mut dyn crate::core::visit::Visitor) {
        for (i, value) in self.iter().enumerate() {
            let key = i.to_string();
            crate::core::visit::perform_visit(visitor, value, Some(&key));
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
