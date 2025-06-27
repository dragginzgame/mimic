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

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Display, FromStr, Serialize)]
#[remain::sorted]
pub enum ConstantType {
    Bool,
    Float32,
    Float64,
    Int8,
    Int16,
    Int32,
    Int64,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Str,
}

impl ConstantType {
    #[must_use]
    pub fn as_type(&self) -> TokenStream {
        use ConstantType::*;

        match self {
            Bool => quote!(bool),
            Float32 => quote!(f32),
            Float64 => quote!(f64),
            Int8 => quote!(i8),
            Int16 => quote!(i16),
            Int32 => quote!(i32),
            Int64 => quote!(i64),
            Nat8 => quote!(u8),
            Nat16 => quote!(u16),
            Nat32 => quote!(u32),
            Nat64 => quote!(u64),
            Str => quote!(&str),
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

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Display, FromStr, Serialize)]
#[remain::sorted]
pub enum PrimitiveType {
    Account,
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
    Nat,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Principal,
    Relation,
    RelationMany,
    Subaccount,
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
        !matches!(self, Self::Blob | Self::RelationMany | Self::Unit)
    }

    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float32 | Self::Float64)
    }

    // is_numeric
    // no floats, this is the check for all the arithmetic traits
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        use PrimitiveType::*;

        matches!(
            self,
            Int | Int8
                | Int16
                | Int32
                | Int64
                | Nat
                | Nat8
                | Nat16
                | Nat32
                | Nat64
                | Float32
                | Float64
                | Decimal
        )
    }

    #[must_use]
    pub fn as_type(&self) -> TokenStream {
        use PrimitiveType::*;

        match self {
            Account => quote!(::mimic::types::Account),
            Bool => quote!(::mimic::types::Bool),
            Blob => quote!(::mimic::types::Blob),
            Decimal => quote!(::mimic::types::Decimal),
            Float32 => quote!(::mimic::types::Float32),
            Float64 => quote!(::mimic::types::Float64),
            Int => quote!(::mimic::types::Int),
            Int8 => quote!(::mimic::types::Int8),
            Int16 => quote!(::mimic::types::Int16),
            Int32 => quote!(::mimic::types::Int32),
            Int64 => quote!(::mimic::types::Int64),
            Principal => quote!(::mimic::types::Principal),
            Relation => quote!(::mimic::types::Relation),
            RelationMany => quote!(::mimic::types::RelationMany),
            Nat => quote!(::mimic::types::Nat),
            Nat8 => quote!(::mimic::types::Nat8),
            Nat16 => quote!(::mimic::types::Nat16),
            Nat32 => quote!(::mimic::types::Nat32),
            Nat64 => quote!(::mimic::types::Nat64),
            Subaccount => quote!(::mimic::types::Subaccount),
            Text => quote!(::mimic::types::Text),
            Unit => quote!(::mimic::types::Unit),
            Ulid => quote!(::mimic::types::Ulid),
        }
    }

    #[must_use]
    pub fn num_cast_fn(self) -> String {
        use PrimitiveType::*;

        match self {
            Float32 => "f32",
            Decimal | Float64 => "f64",
            Int8 => "i8",
            Int16 => "i16",
            Int32 => "i32",
            Int64 => "i64",
            Nat8 => "u8",
            Nat16 => "u16",
            Nat32 => "u32",
            Nat64 => "u64",
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
/// StoreType
///

#[derive(CandidType, Clone, Copy, Debug, Deserialize, Display, FromStr, Serialize)]
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
