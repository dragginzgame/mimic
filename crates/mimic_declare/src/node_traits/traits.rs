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
    CandidType,
    Clone,
    Copy,
    Debug,
    Default,
    Deref,
    DerefMut,
    Deserialize,
    Display,
    Eq,
    Hash,
    Mul,
    MulAssign,
    Ord,
    PartialEq,
    PartialOrd,
    IntoIterator,
    Serialize,
    Sub,
    SubAssign,

    // kind
    // traits for the implementation of specific Schema Nodes
    CanisterKind,
    EntityKind,
    EntityIdKind,
    EnumValueKind,
    IndexKind,
    PrimitiveKind,
    StoreKind,

    // orm
    EntityFixture,
    FieldValue,
    FieldValues,
    From,
    Into,
    NumCast,
    NumFromPrimitive,
    NumToPrimitive,
    Path,
    Sorted,
    TypeView,
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
        Trait::Path,
    ]
});

static TYPE_TRAITS: LazyLock<Vec<Trait>> = LazyLock::new(|| {
    vec![
        Trait::Default,
        Trait::Deserialize,
        Trait::FieldValue,
        Trait::From,
        Trait::PartialEq,
        Trait::Serialize,
        Trait::TypeView,
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
            Self::Hash => Some(quote!(Hash)),
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

    pub fn derive_attribute(self) -> Option<TokenStream> {
        match self {
            Self::Sorted => Some(quote!(#[::mimic::export::remain::sorted])),
            Self::Default => Some(quote!(#[serde(default)])),
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

        quote!(::mimic::core::traits::#trait_name).to_tokens(tokens);
    }
}

///
/// Traits
///

#[derive(Clone, Debug, Default, FromMeta)]
pub struct Traits {
    #[darling(default)]
    pub add: TraitList,

    #[darling(default)]
    pub remove: TraitList,
}

impl Traits {
    // new
    pub fn new() -> Self {
        Self::default()
    }

    // with_path_trait
    pub fn with_path_trait(mut self) -> Self {
        self.add(Trait::Path);
        self
    }

    // with_default_traits
    pub fn with_default_traits(mut self) -> Self {
        self.add.extend(DEFAULT_TRAITS.to_vec());
        self
    }

    // with_type_traits
    pub fn with_type_traits(mut self) -> Self {
        self.add.extend(DEFAULT_TRAITS.to_vec());
        self.add.extend(TYPE_TRAITS.to_vec());
        self
    }

    // add
    pub fn add(&mut self, tr: Trait) {
        self.add.push(tr);
    }

    // extend
    pub fn extend(&mut self, traits: Vec<Trait>) {
        self.add.extend(traits);
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
/// TraitList
///

#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct TraitList(pub Vec<Trait>);

impl TraitList {
    pub fn to_derive_tokens(&self) -> TokenStream {
        if self.0.is_empty() {
            quote!()
        } else {
            let derive_paths = self.0.iter().filter_map(|tr| tr.derive_path());

            quote! {
                #[derive(#(#derive_paths),*)]
            }
        }
    }
}

impl From<&[Trait]> for TraitList {
    fn from(traits: &[Trait]) -> Self {
        Self(traits.to_vec())
    }
}

impl FromMeta for TraitList {
    fn from_list(items: &[NestedMeta]) -> Result<Self, DarlingError> {
        let mut traits = Self::default();

        for item in items {
            let tr = Trait::from_nested_meta(item)?;
            traits.push(tr);
        }

        Ok(traits)
    }
}
