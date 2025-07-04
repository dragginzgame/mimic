use darling::FromMeta;
use derive_more::{Deref, DerefMut};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

pub use mimic_schema::types::*;

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
/// BPrimitive
///

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct BPrimitive(pub Primitive);

impl BPrimitive {
    #[must_use]
    pub fn supports_arithmetic(self) -> bool {
        self.is_int() || self.is_fixed_point() || self.is_decimal()
    }

    pub fn supports_copy(self) -> bool {
        matches!(*self, Primitive::Bool | Primitive::Principal) || self.is_numeric()
    }

    #[must_use]
    pub fn supports_display(self) -> bool {
        !matches!(*self, Primitive::Blob | Primitive::Unit)
    }

    pub fn supports_eq(self) -> bool {
        !self.is_float()
    }

    pub fn supports_hash(self) -> bool {
        !(self.is_float() || matches!(*self, Primitive::Blob))
    }

    pub fn supports_num_cast(self) -> bool {
        matches!(
            *self,
            Primitive::Decimal
                | Primitive::E8s
                | Primitive::E18s
                | Primitive::Int
                | Primitive::Int8
                | Primitive::Int16
                | Primitive::Int32
                | Primitive::Int64
                | Primitive::Float32
                | Primitive::Float64
                | Primitive::Nat
                | Primitive::Nat8
                | Primitive::Nat16
                | Primitive::Nat32
                | Primitive::Nat64
        )
    }

    pub fn supports_partial_ord(self) -> bool {
        !matches!(*self, Primitive::Blob | Primitive::Unit)
    }

    pub fn supports_total_ord(self) -> bool {
        !matches!(
            *self,
            Primitive::Blob
                | Primitive::Decimal
                | Primitive::Float32
                | Primitive::Float64
                | Primitive::Unit
        )
    }

    //
    // grouped helpers
    //

    pub fn is_decimal(self) -> bool {
        matches!(*self, Primitive::Decimal)
    }

    // is_numeric
    // no floats, this is the check for all the arithmetic traits
    pub fn is_numeric(self) -> bool {
        self.is_int() || self.is_float() || self.is_fixed_point() || self.is_decimal()
    }

    pub fn is_float(self) -> bool {
        matches!(*self, Primitive::Float32 | Primitive::Float64)
    }

    pub fn is_signed_int(self) -> bool {
        matches!(
            *self,
            Primitive::Int
                | Primitive::Int8
                | Primitive::Int16
                | Primitive::Int32
                | Primitive::Int64
        )
    }

    pub fn is_unsigned_int(self) -> bool {
        matches!(
            *self,
            Primitive::Nat
                | Primitive::Nat8
                | Primitive::Nat16
                | Primitive::Nat32
                | Primitive::Nat64
        )
    }

    pub fn is_int(self) -> bool {
        self.is_signed_int() || self.is_unsigned_int()
    }

    pub fn is_fixed_point(self) -> bool {
        matches!(*self, Primitive::E8s | Primitive::E18s)
    }

    #[must_use]
    pub fn as_type(self) -> TokenStream {
        match &*self {
            Primitive::Account => quote!(::mimic::core::types::Account),
            Primitive::Bool => quote!(::mimic::core::types::Bool),
            Primitive::Blob => quote!(::mimic::core::types::Blob),
            Primitive::Decimal => quote!(::mimic::core::types::Decimal),
            Primitive::E8s => quote!(::mimic::core::types::E8s),
            Primitive::E18s => quote!(::mimic::core::types::E18s),
            Primitive::Float32 => quote!(::mimic::core::types::Float32),
            Primitive::Float64 => quote!(::mimic::core::types::Float64),
            Primitive::Int => quote!(::mimic::core::types::Int),
            Primitive::Int8 => quote!(::mimic::core::types::Int8),
            Primitive::Int16 => quote!(::mimic::core::types::Int16),
            Primitive::Int32 => quote!(::mimic::core::types::Int32),
            Primitive::Int64 => quote!(::mimic::core::types::Int64),
            Primitive::Principal => quote!(::mimic::core::types::Principal),
            Primitive::Nat => quote!(::mimic::core::types::Nat),
            Primitive::Nat8 => quote!(::mimic::core::types::Nat8),
            Primitive::Nat16 => quote!(::mimic::core::types::Nat16),
            Primitive::Nat32 => quote!(::mimic::core::types::Nat32),
            Primitive::Nat64 => quote!(::mimic::core::types::Nat64),
            Primitive::Subaccount => quote!(::mimic::core::types::Subaccount),
            Primitive::Text => quote!(::mimic::core::types::Text),
            Primitive::Unit => quote!(::mimic::core::types::Unit),
            Primitive::Ulid => quote!(::mimic::core::types::Ulid),
        }
    }

    #[must_use]
    pub fn num_cast_fn(self) -> String {
        match &*self {
            Primitive::E18s => "u128",
            Primitive::Float32 => "f32",
            Primitive::Decimal | Primitive::Float64 => "f64",
            Primitive::Int8 => "i8",
            Primitive::Int16 => "i16",
            Primitive::Int32 => "i32",
            Primitive::Int64 => "i64",
            Primitive::Nat8 => "u8",
            Primitive::Nat16 => "u16",
            Primitive::Nat32 => "u32",
            Primitive::Nat64 | Primitive::E8s => "u64",
            _ => panic!("unexpected primitive type"),
        }
        .into()
    }
}

impl FromMeta for BPrimitive {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        let inner = s
            .parse::<Primitive>()
            .map_err(|_| darling::Error::unknown_value(s))?;

        Ok(Self(inner))
    }
}

impl ToTokens for BPrimitive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::Primitive::#ident));
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
