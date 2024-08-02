use candid::CandidType;
use darling::FromMeta;
use derive_more::{Add, Deref, DerefMut, Sub};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use schema::Schemable;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use syn::Lit;

///
/// Cardinality
///

#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    Deserialize,
    Display,
    EnumString,
    Eq,
    FromMeta,
    PartialEq,
    Serialize,
)]
pub enum Cardinality {
    #[default]
    One,
    Opt,
    Many,
}

impl Schemable for Cardinality {
    fn schema(&self) -> TokenStream {
        match &self {
            Self::One => quote!(::mimic::orm::schema::types::Cardinality::One),
            Self::Opt => quote!(::mimic::orm::schema::types::Cardinality::Opt),
            Self::Many => quote!(::mimic::orm::schema::types::Cardinality::Many),
        }
    }
}

///
/// CrudAction
///

#[derive(
    Clone, Copy, Debug, Deserialize, Display, EnumString, Eq, FromMeta, PartialEq, Serialize,
)]
pub enum CrudAction {
    Load,
    Save,
    Delete,
}

impl Schemable for CrudAction {
    fn schema(&self) -> TokenStream {
        match &self {
            Self::Load => quote!(::mimic::orm::schema::types::CrudAction::Load),
            Self::Save => quote!(::mimic::orm::schema::types::CrudAction::Save),
            Self::Delete => quote!(::mimic::orm::schema::types::CrudAction::Delete),
        }
    }
}

///
/// Cycles
///

#[derive(
    Add,
    CandidType,
    Clone,
    Debug,
    Deref,
    DerefMut,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    Sub,
)]
pub struct Cycles(pub u128);

impl From<u128> for Cycles {
    fn from(n: u128) -> Self {
        Self(n)
    }
}

impl FromMeta for Cycles {
    fn from_value(value: &Lit) -> darling::Result<Self> {
        match value {
            Lit::Str(lit_str) => {
                let val = parse_cycles(&lit_str.value())?;
                Ok(Self(val))
            }
            _ => Err(darling::Error::unexpected_lit_type(value)),
        }
    }
}

impl Schemable for Cycles {
    fn schema(&self) -> TokenStream {
        let n = &self.0;
        quote!(#n)
    }
}

impl ToTokens for Cycles {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let cycles = self.0;
        tokens.extend(quote!(#cycles));
    }
}

// parse_cycles
// helper function to parse string with multiplier suffix
fn parse_cycles(value: &str) -> darling::Result<u128> {
    let mut num_str = String::new();
    let mut suffix_str = String::new();
    let mut seen_dot = false;

    for c in value.chars() {
        if c.is_ascii_digit() || (c == '.' && !seen_dot) {
            if c == '.' {
                seen_dot = true;
            }
            num_str.push(c);
        } else {
            suffix_str.push(c);
        }
    }

    let number: f64 = num_str
        .parse()
        .map_err(|_| darling::Error::custom("cannot parse number part into f64"))?;

    let multiplier = match suffix_str.as_str() {
        "K" => 1_000_f64,
        "M" => 1_000_000_f64,
        "B" => 1_000_000_000_f64,
        "T" => 1_000_000_000_000_f64,
        "Q" => 1_000_000_000_000_000_f64,
        _ => 1_f64,
    };

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    Ok((number * multiplier) as u128)
}

///
/// PrimitiveType
///

#[derive(
    Debug, Clone, Copy, Deserialize, Display, EnumString, Eq, Hash, PartialEq, PartialOrd, Serialize,
)]
#[remain::sorted]
pub enum PrimitiveType {
    Blob,
    Bool,
    Decimal,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    I128,
    Principal,
    String,
    Timestamp,
    U8,
    U16,
    U32,
    U64,
    U128,
    Ulid,
}

impl PrimitiveType {
    #[must_use]
    pub const fn is_orderable(&self) -> bool {
        !matches!(*self, Self::Blob | Self::F32 | Self::F64)
    }

    #[must_use]
    pub const fn group(&self) -> PrimitiveGroup {
        match self {
            Self::Blob => PrimitiveGroup::Blob,
            Self::Bool => PrimitiveGroup::Bool,
            Self::F32 | Self::F64 => PrimitiveGroup::Float,
            Self::Timestamp
            | Self::I8
            | Self::I16
            | Self::I32
            | Self::I64
            | Self::I128
            | Self::U8
            | Self::U16
            | Self::U32
            | Self::U64
            | Self::U128 => PrimitiveGroup::Integer,
            Self::String | Self::Ulid | Self::Principal => PrimitiveGroup::String,
            Self::Decimal => PrimitiveGroup::Decimal,
        }
    }

    #[must_use]
    pub fn num_cast_fn(&self) -> String {
        match self {
            Self::F32 => "f32",
            Self::Decimal | Self::F64 => "f64",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 | Self::Timestamp => "u64",
            Self::U128 => "u128",
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

impl Schemable for PrimitiveType {
    fn schema(&self) -> TokenStream {
        let ident = format_ident!("{}", self.to_string());

        quote!(::mimic::orm::schema::types::PrimitiveType::#ident)
    }
}

impl ToTokens for PrimitiveType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = match self {
            Self::Bool => quote!(bool),
            Self::Blob => quote!(::base::types::Blob),
            Self::Decimal => quote!(::base::types::Decimal),
            Self::F32 => quote!(f32),
            Self::F64 => quote!(f64),
            Self::I8 => quote!(i8),
            Self::I16 => quote!(i16),
            Self::I32 => quote!(i32),
            Self::I64 => quote!(i64),
            Self::I128 => quote!(i128),
            Self::Principal => quote!(::base::types::Principal),
            Self::String => quote!(String),
            Self::Timestamp => quote!(::base::types::Timestamp),
            Self::Ulid => quote!(::base::types::Ulid),
            Self::U8 => quote!(u8),
            Self::U16 => quote!(u16),
            Self::U32 => quote!(u32),
            Self::U64 => quote!(u64),
            Self::U128 => quote!(u128),
        };
        tokens.extend(ty);
    }
}

///
/// PrimitiveGroup
///

#[derive(
    Debug, Clone, Copy, Deserialize, Display, EnumString, Eq, Hash, PartialEq, PartialOrd, Serialize,
)]
#[remain::sorted]
pub enum PrimitiveGroup {
    Blob,
    Bool,
    Decimal,
    Float,
    Integer,
    String,
}

impl PrimitiveGroup {
    #[must_use]
    pub const fn is_orderable(&self) -> bool {
        !matches!(*self, Self::Blob | Self::Float)
    }
}

impl FromMeta for PrimitiveGroup {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        s.parse().map_err(|_| darling::Error::unknown_value(s))
    }
}

impl Schemable for PrimitiveGroup {
    fn schema(&self) -> TokenStream {
        let ident = format_ident!("{}", self.to_string());

        quote!(::mimic::orm::schema::types::PrimitiveGroup::#ident)
    }
}

///
/// SortDirection
///

#[derive(
    CandidType,
    Clone,
    Copy,
    Default,
    Debug,
    Deserialize,
    Display,
    EnumString,
    Eq,
    FromMeta,
    PartialEq,
    Serialize,
)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

impl Schemable for SortDirection {
    fn schema(&self) -> TokenStream {
        match &self {
            Self::Asc => quote!(::mimic::orm::schema::types::SortDirection::Asc),
            Self::Desc => quote!(::mimic::orm::schema::types::SortDirection::Desc),
        }
    }
}

impl ToTokens for SortDirection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = match self {
            Self::Asc => quote!(::mimic::orm::types::SortDirection::Asc),
            Self::Desc => quote!(::mimic::orm::types::SortDirection::Desc),
        };
        tokens.extend(ts);
    }
}

///
/// Sorted
///

#[derive(Clone, Debug, Default, FromMeta)]
pub struct Sorted(bool);

impl ToTokens for Sorted {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.0 {
            tokens.extend(quote!(#[remain::sorted]));
        }
    }
}
