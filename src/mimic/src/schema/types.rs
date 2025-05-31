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
    Nat,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Nat128,
    Principal,
    Relation,
    RelationSet,
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
    pub fn as_type(&self) -> TokenStream {
        match self {
            Self::Bool => quote!(bool),
            Self::Blob => quote!(::mimic::types::prim::Blob),
            Self::Decimal => quote!(::mimic::types::prim::Decimal),
            Self::Float32 => quote!(f32),
            Self::Float64 => quote!(f64),
            Self::Int => quote!(::mimic::types::prim::Int),
            Self::Int8 => quote!(i8),
            Self::Int16 => quote!(i16),
            Self::Int32 => quote!(i32),
            Self::Int64 => quote!(i64),
            Self::Int128 => quote!(i128),
            Self::Principal => quote!(::mimic::types::prim::Principal),
            Self::Nat => quote!(::mimic::types::prim::Nat),
            Self::Nat8 => quote!(u8),
            Self::Nat16 => quote!(u16),
            Self::Nat32 => quote!(u32),
            Self::Nat64 => quote!(u64),
            Self::Nat128 => quote!(u128),
            Self::Relation => quote!(::mimic::types::prim::Relation),
            Self::RelationSet => quote!(::mimic::types::prim::RelationSet),
            Self::Text => quote!(::std::string::String),
            Self::Unit => quote!(::mimic::types::prim::Unit),
            Self::Ulid => quote!(::mimic::types::prim::Ulid),
        }
    }

    #[must_use]
    pub const fn group(self) -> PrimitiveGroup {
        match self {
            Self::Blob => PrimitiveGroup::Blob,
            Self::Bool => PrimitiveGroup::Bool,
            Self::Decimal => PrimitiveGroup::Decimal,
            Self::Float32 | Self::Float64 => PrimitiveGroup::Float,
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
            | Self::Nat128 => PrimitiveGroup::Integer,
            Self::Relation | Self::RelationSet => PrimitiveGroup::Relation,
            Self::Text | Self::Principal => PrimitiveGroup::Text,
            Self::Ulid => PrimitiveGroup::Ulid,
            Self::Unit => PrimitiveGroup::Unit,
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
/// PrimitiveGroup
///

#[derive(CandidType, Debug, Clone, Copy, Deserialize, Display, FromStr, Serialize)]
#[remain::sorted]
pub enum PrimitiveGroup {
    Blob,
    Bool,
    Decimal,
    Float,
    Integer,
    Relation,
    Text,
    Ulid,
    Unit,
}

impl FromMeta for PrimitiveGroup {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        s.parse().map_err(|_| darling::Error::unknown_value(s))
    }
}

impl ToTokens for PrimitiveGroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = format_ident!("{}", self.to_string());

        tokens.extend(quote!(::mimic::schema::types::PrimitiveGroup::#ident));
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
