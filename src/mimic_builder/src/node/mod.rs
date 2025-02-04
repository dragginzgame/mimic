mod arg;
mod constant;
mod def;
mod entity;
mod entity_id;
mod r#enum;
mod enum_value;
mod field;
mod index;
mod item;
mod map;
mod newtype;
mod primitive;
mod record;
mod selector;
mod sort_key;
mod traits;
mod tuple;
mod r#type;
mod validator;
mod value;

// mostly just one or two types in each file so wildcard should be ok
pub use self::arg::*;
pub use self::constant::*;
pub use self::def::*;
pub use self::entity::*;
pub use self::entity_id::*;
pub use self::enum_value::*;
pub use self::field::*;
pub use self::index::*;
pub use self::item::*;
pub use self::map::*;
pub use self::newtype::*;
pub use self::primitive::*;
pub use self::r#enum::*;
pub use self::r#type::*;
pub use self::record::*;
pub use self::selector::*;
pub use self::sort_key::*;
pub use self::traits::*;
pub use self::tuple::*;
pub use self::validator::*;
pub use self::value::*;

use crate::traits::Schemable;
use darling::FromMeta;
use derive_more::{Add, Deref, DerefMut, Display, FromStr, Sub};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
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

pub trait TraitNode: MacroNode {
    // traits
    // for each type this should return the list of traits it requires
    // want to make the function explicit to make it less confusing
    fn traits(&self) -> Vec<Trait>;

    // derive
    fn derive(&self) -> TokenStream {
        let mut derives = Vec::new();

        // map_derive checks if we should derive it
        for t in self.traits() {
            if let Some(path) = t.derive_path() {
                if self.map_derive(t) {
                    derives.push(path);
                }
            }
        }

        if derives.is_empty() {
            quote!()
        } else {
            quote! {
                #[derive(#(#derives),*)]
            }
        }
    }

    // derive_struct
    // includes the extra attributes that a struct needs
    fn derive_struct(&self) -> TokenStream {
        let mut q = self.derive();

        // attributes
        if self.traits().contains(&Trait::Default) {
            q.extend(quote! {
                #[serde(default)]
            });
        }

        q
    }

    // map_derive
    // should a deriveable trait be derived?
    fn map_derive(&self, _: Trait) -> bool {
        true
    }

    /// imp
    /// every trait that returns Some(tokens) is an impl block
    fn imp(&self) -> TokenStream {
        let mut output = quote!();

        for t in self.traits() {
            output.extend(self.map_imp(t));
        }

        output
    }

    // map_imp
    // passes through the trait to the impl generator function
    fn map_imp(&self, t: Trait) -> TokenStream;
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
            Self::One => quote!(::mimic::types::Cardinality::One),
            Self::Opt => quote!(::mimic::types::Cardinality::Opt),
            Self::Many => quote!(::mimic::types::Cardinality::Many),
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
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    Str,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
}

impl FromMeta for ConstantType {
    fn from_string(s: &str) -> Result<Self, darling::Error> {
        s.parse().map_err(|_| darling::Error::unknown_value(s))
    }
}

impl Schemable for ConstantType {
    fn schema(&self) -> TokenStream {
        let ident = format_ident!("{}", self.to_string());

        quote!(::mimic::types::ConstantType::#ident)
    }
}

impl ToTokens for ConstantType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = match self {
            Self::Bool => quote!(bool),
            Self::I8 => quote!(i8),
            Self::I16 => quote!(i16),
            Self::I32 => quote!(i32),
            Self::I64 => quote!(i64),
            Self::I128 => quote!(i128),
            Self::Isize => quote!(isize),
            Self::Str => quote!(&str),
            Self::U8 => quote!(u8),
            Self::U16 => quote!(u16),
            Self::U32 => quote!(u32),
            Self::U64 => quote!(u64),
            Self::U128 => quote!(u128),
            Self::Usize => quote!(usize),
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
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    Principal,
    String,
    Todo,
    U8,
    U16,
    U32,
    U64,
    U128,
    Ulid,
    Unit,
    Usize,
}

impl PrimitiveType {
    #[must_use]
    pub const fn is_orderable(self) -> bool {
        !matches!(self, Self::Blob | Self::F32 | Self::F64)
    }

    #[must_use]
    pub const fn group(self) -> PrimitiveGroup {
        match self {
            Self::Blob => PrimitiveGroup::Blob,
            Self::Bool | Self::Todo => PrimitiveGroup::Bool,
            Self::Decimal => PrimitiveGroup::Decimal,
            Self::F32 | Self::F64 => PrimitiveGroup::Float,
            Self::I8
            | Self::I16
            | Self::I32
            | Self::I64
            | Self::I128
            | Self::Isize
            | Self::U8
            | Self::U16
            | Self::U32
            | Self::U64
            | Self::U128
            | Self::Usize => PrimitiveGroup::Integer,
            Self::String | Self::Principal => PrimitiveGroup::String,
            Self::Ulid => PrimitiveGroup::Ulid,
            Self::Unit => PrimitiveGroup::Unit,
        }
    }

    #[must_use]
    pub fn num_cast_fn(self) -> String {
        match self {
            Self::F32 => "f32",
            Self::Decimal | Self::F64 => "f64",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::Isize => "isize",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::Usize => "usize",
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

        quote!(::mimic::types::PrimitiveType::#ident)
    }
}

impl ToTokens for PrimitiveType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = match self {
            Self::Bool => quote!(bool),
            Self::Blob => quote!(::mimic::orm::base::types::Blob),
            Self::Decimal => quote!(::mimic::orm::base::types::Decimal),
            Self::F32 => quote!(f32),
            Self::F64 => quote!(f64),
            Self::I8 => quote!(i8),
            Self::I16 => quote!(i16),
            Self::I32 => quote!(i32),
            Self::I64 => quote!(i64),
            Self::I128 => quote!(i128),
            Self::Isize => quote!(isize),
            Self::Principal => quote!(::mimic::orm::base::types::Principal),
            Self::String => quote!(String),
            Self::Todo => quote!(::mimic::orm::base::types::Todo),
            Self::U8 => quote!(u8),
            Self::U16 => quote!(u16),
            Self::U32 => quote!(u32),
            Self::U64 => quote!(u64),
            Self::U128 => quote!(u128),
            Self::Unit => quote!(::mimic::orm::base::types::Unit),
            Self::Ulid => quote!(::mimic::orm::base::types::Ulid),
            Self::Usize => quote!(usize),
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

        quote!(::mimic::types::PrimitiveGroup::#ident)
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
            Self::Asc => quote!(::mimic::types::SortDirection::Asc),
            Self::Desc => quote!(::mimic::types::SortDirection::Desc),
        }
    }
}

impl ToTokens for SortDirection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = match self {
            Self::Asc => quote!(::mimic::types::SortDirection::Asc),
            Self::Desc => quote!(::mimic::types::SortDirection::Desc),
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
