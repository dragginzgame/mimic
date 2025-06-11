use candid::CandidType;
use darling::FromMeta;
use derive_more::{Display, FromStr};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use serde::{Deserialize, Serialize};

///
/// Cardinality
///

#[derive(
    CandidType,
    Clone,
    Copy,
    Default,
    Debug,
    Deserialize,
    Display,
    Eq,
    FromMeta,
    FromStr,
    PartialEq,
    Serialize,
)]
pub enum Cardinality {
    #[default]
    One,
    Opt,
    Many,
}

impl ToTokens for Cardinality {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::Cardinality::#ident));
    }
}

///
/// ConstantType
///

#[derive(CandidType, Debug, Clone, Copy, Deserialize, Display, FromStr, Serialize)]
#[remain::sorted]
pub enum ConstantType {
    Bool,
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Nat128,
    Str,
}

impl ConstantType {
    #[must_use]
    pub fn as_type(&self) -> TokenStream {
        match self {
            Self::Bool => quote!(bool),
            Self::Float32 => quote!(f32),
            Self::Float64 => quote!(f64),
            Self::Int8 => quote!(i8),
            Self::Int16 => quote!(i16),
            Self::Int32 => quote!(i32),
            Self::Int64 => quote!(i64),
            Self::Int128 => quote!(i128),
            Self::Nat8 => quote!(u8),
            Self::Nat16 => quote!(u16),
            Self::Nat32 => quote!(u32),
            Self::Nat64 => quote!(u64),
            Self::Nat128 => quote!(u128),
            Self::Str => quote!(&str),
        }
    }
}

impl FromMeta for ConstantType {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        s.parse().map_err(|_| darling::Error::unknown_value(s))
    }
}

impl ToTokens for ConstantType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::ConstantType::#ident));
    }
}

///
/// PrimitiveType
///

#[derive(CandidType, Debug, Clone, Copy, Display, FromStr, Serialize, Deserialize)]
#[remain::sorted]
pub enum PrimitiveType {
    Blob,
    Bool,
    Decimal,
    Float32,
    Float64,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Key,
    KeySet,
    Nat,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Nat128,
    Principal,
    Text,
    Ulid,
    Unit,
}

impl PrimitiveType {
    #[must_use]
    pub const fn is_orderable(self) -> bool {
        !matches!(self, Self::Blob | Self::Float32 | Self::Float64)
    }

    #[must_use]
    pub const fn is_displayable(self) -> bool {
        !matches!(self, Self::Blob | Self::KeySet | Self::Unit)
    }

    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float32 | Self::Float64)
    }

    // is_numeric
    // no floats, this is the check for all the arithmetic traits
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::Int
                | Self::Int8
                | Self::Int16
                | Self::Int32
                | Self::Int64
                | Self::Int128
                | Self::Nat
                | Self::Nat8
                | Self::Nat16
                | Self::Nat32
                | Self::Nat64
                | Self::Nat128
                | Self::Float32
                | Self::Float64
                | Self::Decimal
        )
    }

    #[must_use]
    pub fn as_type(&self) -> TokenStream {
        match self {
            Self::Bool => quote!(::mimic::types::Bool),
            Self::Blob => quote!(::mimic::types::Blob),
            Self::Decimal => quote!(::mimic::types::Decimal),
            Self::Float32 => quote!(::mimic::types::Float32),
            Self::Float64 => quote!(::mimic::types::Float64),
            Self::Int => quote!(::mimic::types::Int),
            Self::Int8 => quote!(::mimic::types::Int8),
            Self::Int16 => quote!(::mimic::types::Int16),
            Self::Int32 => quote!(::mimic::types::Int32),
            Self::Int64 => quote!(::mimic::types::Int64),
            Self::Int128 => quote!(::mimic::types::Int128),
            Self::Key => quote!(::mimic::types::Key),
            Self::KeySet => quote!(::mimic::types::KeySet),
            Self::Principal => quote!(::mimic::types::Principal),
            Self::Nat => quote!(::mimic::types::Nat),
            Self::Nat8 => quote!(::mimic::types::Nat8),
            Self::Nat16 => quote!(::mimic::types::Nat16),
            Self::Nat32 => quote!(::mimic::types::Nat32),
            Self::Nat64 => quote!(::mimic::types::Nat64),
            Self::Nat128 => quote!(::mimic::types::Nat128),
            Self::Text => quote!(::mimic::types::Text),
            Self::Unit => quote!(::mimic::types::Unit),
            Self::Ulid => quote!(::mimic::types::Ulid),
        }
    }

    #[must_use]
    pub fn num_cast_fn(self) -> String {
        match self {
            Self::Float32 => "f32",
            Self::Decimal | Self::Float64 => "f64",
            Self::Int8 => "i8",
            Self::Int16 => "i16",
            Self::Int32 => "i32",
            Self::Int64 => "i64",
            Self::Int128 => "i128",
            Self::Nat8 => "u8",
            Self::Nat16 => "u16",
            Self::Nat32 => "u32",
            Self::Nat64 => "u64",
            Self::Nat128 => "u128",
            _ => panic!("unexpected primitive type"),
        }
        .into()
    }
}

impl FromMeta for PrimitiveType {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        s.parse().map_err(|_| darling::Error::unknown_value(s))
    }
}

impl ToTokens for PrimitiveType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::PrimitiveType::#ident));
    }
}

///
/// SortDirection
///

#[derive(
    CandidType, Clone, Copy, Default, Debug, Deserialize, Display, FromMeta, FromStr, Serialize,
)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

impl ToTokens for SortDirection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::SortDirection::#ident));
    }
}

///
/// StoreType
///

#[derive(CandidType, Debug, Clone, Copy, Deserialize, Display, FromStr, Serialize)]
pub enum StoreType {
    Data,
    Index,
}

impl FromMeta for StoreType {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        s.parse().map_err(|_| darling::Error::unknown_value(s))
    }
}

impl ToTokens for StoreType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::StoreType::#ident));
    }
}
