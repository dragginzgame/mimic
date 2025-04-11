mod arg;
mod canister;
mod constant;
mod def;
mod entity;
mod entity_id;
mod r#enum;
mod enum_value;
mod field;
mod index;
mod item;
mod list;
mod map;
mod newtype;
mod primitive;
mod record;
mod selector;
mod set;
mod sort_key;
mod store;
mod traits;
mod tuple;
mod r#type;
mod validator;
mod value;

// mostly just one or two types in each file so wildcard should be ok
pub use self::arg::*;
pub use self::canister::*;
pub use self::constant::*;
pub use self::def::*;
pub use self::entity::*;
pub use self::entity_id::*;
pub use self::r#enum::*;
pub use self::enum_value::*;
pub use self::field::*;
pub use self::index::*;
pub use self::item::*;
pub use self::list::*;
pub use self::map::*;
pub use self::newtype::*;
pub use self::primitive::*;
pub use self::record::*;
pub use self::selector::*;
pub use self::set::*;
pub use self::sort_key::*;
pub use self::store::*;
pub use self::traits::*;
pub use self::tuple::*;
pub use self::r#type::*;
pub use self::validator::*;
pub use self::value::*;

use crate::traits::Schemable;
use darling::FromMeta;
use derive_more::{Add, Deref, DerefMut, Display, FromStr, Sub};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use serde::{Deserialize, Serialize};
use syn::Lit;

///
/// NODE TRAITS
///

///
/// Node
///

pub trait Node {
    fn expand(&self) -> TokenStream;
}

///
/// MacroNode
///

pub trait MacroNode {
    fn def(&self) -> &Def;
}

///
/// TraitNode
///

pub struct TraitTokens {
    pub derive: TokenStream,
    pub impls: TokenStream,
}

pub trait TraitNode: MacroNode {
    // traits
    // returns the list of traits for this type
    fn traits(&self) -> Vec<Trait>;

    // trait_tokens
    fn trait_tokens(&self) -> TraitTokens {
        let mut impls = quote!();
        let traits = self.traits();

        // we only derive traits that have no map_imp tokens
        let mut derived_traits = Vec::new();
        for t in traits {
            match self.map_trait(t) {
                Some(tokens) => impls.extend(tokens),
                None => derived_traits.push(
                    t.derive_path()
                        .unwrap_or_else(|| panic!("trait '{t}' cannot be derived")),
                ),
            }
        }

        // derive
        let mut derive = if derived_traits.is_empty() {
            quote!()
        } else {
            quote! {
                #[derive(#(#derived_traits),*)]
            }
        };

        // derive attributes
        if let Some(attrs) = self.derive_attributes() {
            derive.extend(attrs);
        }

        TraitTokens { derive, impls }
    }

    // map_trait
    // if None is returned it means that this trait should be derived
    // otherwise it's the code for the implementation
    fn map_trait(&self, t: Trait) -> Option<TokenStream>;

    // derive_attributes
    // extra attributes for the derive
    fn derive_attributes(&self) -> Option<TokenStream> {
        None
    }
}

///
/// NODES
///

///
/// Cardinality
///

#[derive(
    Clone, Copy, Default, Debug, Deserialize, Display, Eq, FromMeta, FromStr, PartialEq, Serialize,
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
            Self::One => quote!(::mimic::schema::types::Cardinality::One),
            Self::Opt => quote!(::mimic::schema::types::Cardinality::Opt),
            Self::Many => quote!(::mimic::schema::types::Cardinality::Many),
        }
    }
}

///
/// ConstantType
///

#[derive(
    Debug, Clone, Copy, Deserialize, Display, Eq, FromStr, Hash, PartialEq, PartialOrd, Serialize,
)]
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

impl FromMeta for ConstantType {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        s.parse().map_err(|_| darling::Error::unknown_value(s))
    }
}

impl Schemable for ConstantType {
    fn schema(&self) -> TokenStream {
        let ident = format_ident!("{}", self.to_string());

        quote!(::mimic::schema::types::ConstantType::#ident)
    }
}

impl ToTokens for ConstantType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = match self {
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
        };
        tokens.extend(ty);
    }
}

///
/// Cycles
///

#[derive(
    Add, Clone, Debug, Deref, DerefMut, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Sub,
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
    Debug, Clone, Copy, Deserialize, Display, Eq, FromStr, Hash, PartialEq, PartialOrd, Serialize,
)]
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
    SortKey,
    String,
    Todo,
    Ulid,
    Unit,
}

impl PrimitiveType {
    #[must_use]
    pub const fn is_orderable(self) -> bool {
        !matches!(self, Self::Blob | Self::Float32 | Self::Float64)
    }

    #[must_use]
    pub const fn group(self) -> PrimitiveGroup {
        match self {
            Self::Blob => PrimitiveGroup::Blob,
            Self::Bool | Self::Todo => PrimitiveGroup::Bool,
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
            Self::SortKey => PrimitiveGroup::SortKey,
            Self::String | Self::Principal => PrimitiveGroup::String,
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

impl Schemable for PrimitiveType {
    fn schema(&self) -> TokenStream {
        let ident = format_ident!("{}", self.to_string());

        quote!(::mimic::schema::types::PrimitiveType::#ident)
    }
}

impl ToTokens for PrimitiveType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = match self {
            Self::Bool => quote!(bool),
            Self::Blob => quote!(::mimic::orm::base::types::Blob),
            Self::Decimal => quote!(::mimic::orm::base::types::Decimal),
            Self::Float32 => quote!(f32),
            Self::Float64 => quote!(f64),
            Self::Int => quote!(::mimic::orm::base::types::Int),
            Self::Int8 => quote!(i8),
            Self::Int16 => quote!(i16),
            Self::Int32 => quote!(i32),
            Self::Int64 => quote!(i64),
            Self::Int128 => quote!(i128),
            Self::Principal => quote!(::mimic::orm::base::types::Principal),
            Self::SortKey => quote!(::mimic::orm::base::types::SortKey),
            Self::String => quote!(String),
            Self::Todo => quote!(::mimic::orm::base::types::Todo),
            Self::Nat => quote!(::mimic::orm::base::types::Nat),
            Self::Nat8 => quote!(u8),
            Self::Nat16 => quote!(u16),
            Self::Nat32 => quote!(u32),
            Self::Nat64 => quote!(u64),
            Self::Nat128 => quote!(u128),
            Self::Unit => quote!(::mimic::orm::base::types::Unit),
            Self::Ulid => quote!(::mimic::orm::base::types::Ulid),
        };
        tokens.extend(ty);
    }
}

///
/// PrimitiveGroup
///

#[derive(
    Debug, Clone, Copy, Deserialize, Display, Eq, FromStr, Hash, PartialEq, PartialOrd, Serialize,
)]
#[remain::sorted]
pub enum PrimitiveGroup {
    Blob,
    Bool,
    Decimal,
    Float,
    Integer,
    SortKey,
    String,
    Ulid,
    Unit,
}

impl FromMeta for PrimitiveGroup {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        s.parse().map_err(|_| darling::Error::unknown_value(s))
    }
}

impl Schemable for PrimitiveGroup {
    fn schema(&self) -> TokenStream {
        let ident = format_ident!("{}", self.to_string());

        quote!(::mimic::schema::types::PrimitiveGroup::#ident)
    }
}

///
/// SortDirection
///

#[derive(
    Clone, Copy, Default, Debug, Deserialize, Display, Eq, FromMeta, FromStr, PartialEq, Serialize,
)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

impl Schemable for SortDirection {
    fn schema(&self) -> TokenStream {
        match &self {
            Self::Asc => quote!(::mimic::schema::types::SortDirection::Asc),
            Self::Desc => quote!(::mimic::schema::types::SortDirection::Desc),
        }
    }
}

impl ToTokens for SortDirection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = match self {
            Self::Asc => quote!(::mimic::schema::types::SortDirection::Asc),
            Self::Desc => quote!(::mimic::schema::types::SortDirection::Desc),
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
            tokens.extend(quote!(#[::mimic::export::remain::sorted]));
        }
    }
}
