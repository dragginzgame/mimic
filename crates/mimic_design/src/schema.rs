use darling::FromMeta;
use derive_more::{Deref, DerefMut};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::{
    sync::{LazyLock, Mutex},
    time::SystemTime,
};
use tinyrand::{Rand, Seeded, StdRand};

pub use mimic_schema::types::*;

///
/// RNG
///
/// Create a static, lazily-initialized StdRng instance wrapped in a Mutex
///

static RNG: LazyLock<Mutex<StdRand>> = LazyLock::new(|| {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    let now_u64 = u64::try_from(now).unwrap();

    Mutex::new(StdRand::seed(now_u64))
});

///
/// Schemable
///
/// Any data structure requires this trait to be part of the ctor structure
/// that populates the Schema
///

pub trait Schemable {
    // schema
    // generates the structure which is passed to the static Schema data structure
    // via the ctor crate
    fn schema(&self) -> TokenStream;

    // ctor_schema
    // formats the code needed to send something via ctor to the schema
    #[must_use]
    fn ctor_schema(&self) -> TokenStream {
        let mut rng = RNG.lock().expect("Failed to lock RNG");
        let ctor_fn = format_ident!("ctor_{}", rng.next_u32());

        let schema = self.schema();
        quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #[::mimic::export::ctor::ctor]
            fn #ctor_fn() {
                ::mimic::schema::build::schema_write().insert_node(
                    #schema
                );
            }
        }
    }
}

///
/// BCardinality
///

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct BCardinality(pub Cardinality);

impl ToTokens for BCardinality {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::Cardinality::#ident));
    }
}

///
/// BConstantType
///

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct BConstantType(pub ConstantType);

impl BConstantType {
    #[must_use]
    pub fn as_type(self) -> TokenStream {
        match &*self {
            ConstantType::Bool => quote!(bool),
            ConstantType::Float32 => quote!(f32),
            ConstantType::Float64 => quote!(f64),
            ConstantType::Int8 => quote!(i8),
            ConstantType::Int16 => quote!(i16),
            ConstantType::Int32 => quote!(i32),
            ConstantType::Int64 => quote!(i64),
            ConstantType::Nat8 => quote!(u8),
            ConstantType::Nat16 => quote!(u16),
            ConstantType::Nat32 => quote!(u32),
            ConstantType::Nat64 => quote!(u64),
            ConstantType::Str => quote!(&str),
        }
    }
}

impl FromMeta for BConstantType {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        let inner = s
            .parse::<ConstantType>()
            .map_err(|_| darling::Error::unknown_value(s))?;

        Ok(Self(inner))
    }
}

impl ToTokens for BConstantType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::ConstantType::#ident));
    }
}

///
/// BPrimitiveType
///

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct BPrimitiveType(pub PrimitiveType);

impl BPrimitiveType {
    #[must_use]
    pub fn supports_arithmetic(self) -> bool {
        self.is_int() || self.is_fixed_point()
    }

    pub fn supports_copy(self) -> bool {
        matches!(*self, PrimitiveType::Bool | PrimitiveType::Principal) || self.is_numeric()
    }

    #[must_use]
    pub fn supports_display(self) -> bool {
        !matches!(*self, PrimitiveType::Blob | PrimitiveType::Unit)
    }

    pub fn supports_eq(self) -> bool {
        !self.is_float()
    }

    pub fn supports_hash(self) -> bool {
        !(self.is_float() || matches!(*self, PrimitiveType::Blob))
    }

    #[must_use]
    pub fn supports_num_cast(self) -> bool {
        matches!(
            *self,
            PrimitiveType::Int
                | PrimitiveType::Int32
                | PrimitiveType::Int64
                | PrimitiveType::Float32
                | PrimitiveType::Float64
                | PrimitiveType::Nat32
                | PrimitiveType::Nat64
                | PrimitiveType::FixedE8
        )
    }

    pub fn supports_partial_ord(self) -> bool {
        !matches!(*self, PrimitiveType::Blob | PrimitiveType::Unit)
    }

    pub fn supports_total_ord(self) -> bool {
        !matches!(
            *self,
            PrimitiveType::Blob
                | PrimitiveType::Float32
                | PrimitiveType::Float64
                | PrimitiveType::Unit
        )
    }

    //
    // grouped helpers
    //

    // is_numeric
    // no floats, this is the check for all the arithmetic traits
    pub fn is_numeric(self) -> bool {
        self.is_int() || self.is_float() || self.is_fixed_point()
    }

    pub fn is_float(self) -> bool {
        matches!(*self, PrimitiveType::Float32 | PrimitiveType::Float64)
    }

    pub fn is_signed_int(self) -> bool {
        matches!(
            *self,
            PrimitiveType::Int
                | PrimitiveType::Int8
                | PrimitiveType::Int16
                | PrimitiveType::Int32
                | PrimitiveType::Int64
        )
    }

    pub fn is_unsigned_int(self) -> bool {
        matches!(
            *self,
            PrimitiveType::Nat
                | PrimitiveType::Nat8
                | PrimitiveType::Nat16
                | PrimitiveType::Nat32
                | PrimitiveType::Nat64
        )
    }

    pub fn is_int(self) -> bool {
        self.is_signed_int() || self.is_unsigned_int()
    }

    pub fn is_fixed_point(self) -> bool {
        matches!(*self, PrimitiveType::FixedE8)
    }

    #[must_use]
    pub fn as_type(self) -> TokenStream {
        match &*self {
            PrimitiveType::Account => quote!(::mimic::core::types::Account),
            PrimitiveType::Bool => quote!(::mimic::core::types::Bool),
            PrimitiveType::Blob => quote!(::mimic::core::types::Blob),
            PrimitiveType::FixedE8 => quote!(::mimic::core::types::FixedE8),
            PrimitiveType::Float32 => quote!(::mimic::core::types::Float32),
            PrimitiveType::Float64 => quote!(::mimic::core::types::Float64),
            PrimitiveType::Int => quote!(::mimic::core::types::Int),
            PrimitiveType::Int8 => quote!(::mimic::core::types::Int8),
            PrimitiveType::Int16 => quote!(::mimic::core::types::Int16),
            PrimitiveType::Int32 => quote!(::mimic::core::types::Int32),
            PrimitiveType::Int64 => quote!(::mimic::core::types::Int64),
            PrimitiveType::Principal => quote!(::mimic::core::types::Principal),
            PrimitiveType::Nat => quote!(::mimic::core::types::Nat),
            PrimitiveType::Nat8 => quote!(::mimic::core::types::Nat8),
            PrimitiveType::Nat16 => quote!(::mimic::core::types::Nat16),
            PrimitiveType::Nat32 => quote!(::mimic::core::types::Nat32),
            PrimitiveType::Nat64 => quote!(::mimic::core::types::Nat64),
            PrimitiveType::Subaccount => quote!(::mimic::core::types::Subaccount),
            PrimitiveType::Text => quote!(::mimic::core::types::Text),
            PrimitiveType::Unit => quote!(::mimic::core::types::Unit),
            PrimitiveType::Ulid => quote!(::mimic::core::types::Ulid),
        }
    }

    #[must_use]
    pub fn num_cast_fn(self) -> String {
        match &*self {
            PrimitiveType::Float32 => "f32",
            PrimitiveType::Float64 => "f64",
            PrimitiveType::Int8 => "i8",
            PrimitiveType::Int16 => "i16",
            PrimitiveType::Int32 => "i32",
            PrimitiveType::Int64 => "i64",
            PrimitiveType::Nat8 => "u8",
            PrimitiveType::Nat16 => "u16",
            PrimitiveType::Nat32 => "u32",
            PrimitiveType::Nat64 | PrimitiveType::FixedE8 => "u64",
            _ => panic!("unexpected primitive type"),
        }
        .into()
    }
}

impl FromMeta for BPrimitiveType {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        let inner = s
            .parse::<PrimitiveType>()
            .map_err(|_| darling::Error::unknown_value(s))?;

        Ok(Self(inner))
    }
}

impl ToTokens for BPrimitiveType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::PrimitiveType::#ident));
    }
}

///
/// BStoreType
///

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct BStoreType(pub StoreType);

impl FromMeta for BStoreType {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        let inner = s
            .parse::<StoreType>()
            .map_err(|_| darling::Error::unknown_value(s))?;

        Ok(Self(inner))
    }
}

impl ToTokens for BStoreType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::StoreType::#ident));
    }
}
