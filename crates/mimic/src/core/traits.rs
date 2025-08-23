// re-exports of other traits
// for the standard traits::X pattern
pub use icu::ic::structures::storable::Storable;
pub use num_traits::{FromPrimitive as NumFromPrimitive, NumCast, ToPrimitive as NumToPrimitive};
pub use serde::{Deserialize, Serialize, de::DeserializeOwned};
pub use std::{
    cmp::{Eq, Ordering, PartialEq},
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
    db::{Db, service::EntityService},
    schema::node::Index,
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

pub trait Kind: Path + 'static {}

impl<T> Kind for T where T: Path + 'static {}

///
/// CanisterKind
///

pub trait CanisterKind: Kind {}

///
/// EntityKind
///

pub trait EntityKind: Kind + TypeKind + EntityLifecycle + FieldValues {
    type Store: StoreKind;
    type Canister: CanisterKind; // Self::Store::Canister shortcut

    const ENTITY_ID: u64;
    const PRIMARY_KEY: &'static str;
    const FIELDS: &'static [&'static str];
    const INDEXES: &'static [&'static Index];

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
    Kind + Clone + Default + Serialize + DeserializeOwned + Validate + Visitable + PartialEq + TypeView
{
}

impl<T> TypeKind for T where
    T: Kind
        + Clone
        + Default
        + DeserializeOwned
        + PartialEq
        + Serialize
        + TypeView
        + Validate
        + Visitable
{
}

///
/// OTHER TRAITS
///

///
/// EntityFixture
/// Trait implemented by enums or helper types that can insert fixture
/// data for an entity into the correct Db.
///

pub trait EntityFixture: EntityKind + Sized {
    /// Override if fixtures are purely self-contained
    #[must_use]
    fn fixtures() -> Vec<Self> {
        Vec::new()
    }

    /// Insert fixtures. Default: use `fixtures()`
    fn insert_fixtures(db: Db<Self::Canister>) {
        for entity in Self::fixtures() {
            EntityService::save_fixture(db, entity.clone());
        }
    }

    fn insert(db: Db<Self::Canister>, entity: Self) {
        EntityService::save_fixture(db, entity);
    }
}

///
/// EntityLifecycle
///

pub trait EntityLifecycle {
    fn touch_created(&mut self, now: u64);
    fn touch_updated(&mut self, now: u64);
}

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
        Value::Text(self.to_string())
    }
}

impl FieldValue for String {
    fn to_value(&self) -> Value {
        Value::Text(self.clone())
    }
}

impl FieldValue for () {
    fn to_value(&self) -> Value {
        Value::Unit
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
/// Path
///
/// any node created via a macro has a Path
/// ie. design::game::rarity::Rarity
///

pub trait Path {
    const PATH: &'static str;
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

pub trait Validate: ValidateAuto + ValidateCustom {}

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
