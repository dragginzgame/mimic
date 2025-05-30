use darling::{Error as DarlingError, FromMeta, ast::NestedMeta};
use derive_more::{Deref, DerefMut, Display, FromStr};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, hash::Hash, str::FromStr, sync::LazyLock};

///
/// Trait
/// right now everything in one big enum
///

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    Eq,
    PartialEq,
    FromStr,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub enum Trait {
    // rust + third party
    Add,
    AddAssign,
    AsRef,
    Clone,
    Copy,
    Debug,
    Default,
    Deref,
    DerefMut,
    Deserialize,
    Display,
    Eq,
    From,
    FromStr,
    Hash,
    Into,
    Mul,
    MulAssign,
    NumCast,
    Ord,
    PartialEq,
    PartialOrd,
    IntoIterator,
    Serialize,
    Sub,
    SubAssign,

    // orm
    CandidType,
    Entity,
    EntityDyn,
    EntityFixture,
    EntityId,
    EntitySearch,
    EntitySort,
    EnumValue,
    Inner,
    NodeDyn,
    NumFromPrimitive,
    NumToPrimitive,
    Orderable,
    Path,
    Searchable,
    SortKeyValue,
    ValidateAuto,
    ValidateCustom,
    Visitable,
}

///
/// Traits
///

#[rustfmt::skip]
static DEFAULT_TRAITS: LazyLock<Vec<Trait>> = LazyLock::new(|| {
    vec![
        Trait::Clone,
        Trait::Debug,
        Trait::NodeDyn,
        Trait::Path,
    ]
});

static TYPE_TRAITS: LazyLock<Vec<Trait>> = LazyLock::new(|| {
    vec![
        Trait::CandidType,
        Trait::Eq,
        Trait::Deserialize,
        Trait::Orderable,
        Trait::PartialEq,
        Trait::Searchable,
        Trait::Serialize,
        Trait::ValidateAuto,
        Trait::ValidateCustom,
        Trait::Visitable,
    ]
});

// path_to_string
#[must_use]
pub fn path_to_string(path: &syn::Path) -> String {
    path.to_token_stream()
        .to_string()
        .replace(' ', "")
        .trim_matches(':')
        .to_string()
}

impl Trait {
    #[must_use]
    #[remain::check]
    pub fn derive_path(self) -> Option<TokenStream> {
        #[remain::sorted]
        match self {
            Self::Add => Some(quote!(::mimic::export::derive_more::Add)),
            Self::AddAssign => Some(quote!(::mimic::export::derive_more::AddAssign)),
            Self::CandidType => Some(quote!(::candid::CandidType)),
            Self::Clone => Some(quote!(Clone)),
            Self::Copy => Some(quote!(Copy)),
            Self::Debug => Some(quote!(Debug)),
            Self::Default => Some(quote!(Default)),
            Self::Deref => Some(quote!(::mimic::export::derive_more::Deref)),
            Self::DerefMut => Some(quote!(::mimic::export::derive_more::DerefMut)),
            Self::Deserialize => Some(quote!(::serde::Deserialize)),
            Self::Display => Some(quote!(::mimic::export::derive_more::Display)),
            Self::Eq => Some(quote!(Eq)),
            Self::From => Some(quote!(::mimic::export::derive_more::From)),
            Self::FromStr => Some(quote!(::mimic::export::derive_more::FromStr)),
            Self::Hash => Some(quote!(Hash)),
            Self::Into => Some(quote!(::mimic::export::derive_more::Into)),
            Self::IntoIterator => Some(quote!(::mimic::export::derive_more::IntoIterator)),
            Self::Mul => Some(quote!(::mimic::export::derive_more::Mul)),
            Self::MulAssign => Some(quote!(::mimic::export::derive_more::MulAssign)),
            Self::Ord => Some(quote!(Ord)),
            Self::PartialEq => Some(quote!(PartialEq)),
            Self::PartialOrd => Some(quote!(PartialOrd)),
            Self::Serialize => Some(quote!(::serde::Serialize)),
            Self::Sub => Some(quote!(::mimic::export::derive_more::Sub)),
            Self::SubAssign => Some(quote!(::mimic::export::derive_more::SubAssign)),

            _ => None,
        }
    }
}

impl FromMeta for Trait {
    fn from_nested_meta(item: &NestedMeta) -> Result<Self, DarlingError> {
        match item {
            NestedMeta::Meta(syn::Meta::Path(path)) => {
                let path_str = path_to_string(path);

                Self::from_str(&path_str).map_err(DarlingError::custom)
            }

            _ => Err(DarlingError::custom(format!(
                "expected Meta Path, got {item:?}"
            ))),
        }
    }
}

impl ToTokens for Trait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let trait_name = format_ident!("{}", self.to_string());

        quote!(::mimic::traits::#trait_name).to_tokens(tokens);
    }
}

///
/// Traits
///

#[derive(Clone, Debug, Default, FromMeta)]
pub struct Traits {
    #[darling(default)]
    pub add: TraitAdd,

    #[darling(default)]
    pub remove: TraitRemove,
}

impl Traits {
    // add_type_traits
    pub fn add_type_traits(&mut self) {
        self.add.extend(TYPE_TRAITS.to_vec());
    }

    // add
    pub fn add(&mut self, tr: Trait) {
        self.add.push(tr);
    }

    // extend
    pub fn extend(&mut self, traits: Vec<Trait>) {
        for tr in traits {
            self.add(tr);
        }
    }

    // list
    // generates the TraitList based on the defaults plus traits that have been added or removed
    pub fn list(&self) -> Vec<Trait> {
        let mut traits = HashSet::new();

        // self.add
        for tr in self.add.iter() {
            assert!(traits.insert(*tr), "adding duplicate trait '{tr}'");
        }

        // self.remove
        for tr in self.remove.iter() {
            assert!(
                traits.remove(tr),
                "cannot remove trait {tr} from {traits:?}",
            );
        }

        // sort
        let mut sorted_traits: Vec<Trait> = traits.into_iter().collect();
        sorted_traits.sort();

        sorted_traits
    }
}

///
/// TraitAdd
/// defaults with the common types
///

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct TraitAdd(Vec<Trait>);

impl Default for TraitAdd {
    fn default() -> Self {
        Self(DEFAULT_TRAITS.to_vec())
    }
}

impl FromMeta for TraitAdd {
    fn from_list(items: &[NestedMeta]) -> Result<Self, DarlingError> {
        let mut traits = Self::default();

        for item in items {
            let tr = Trait::from_nested_meta(item)?;
            traits.push(tr);
        }

        Ok(traits)
    }
}

///
/// TraitRemove
///

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct TraitRemove(Vec<Trait>);

impl FromMeta for TraitRemove {
    fn from_list(items: &[NestedMeta]) -> Result<Self, DarlingError> {
        items
            .iter()
            .map(Trait::from_nested_meta)
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }
}
