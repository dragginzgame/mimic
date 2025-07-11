use crate::{
    node::Def,
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

///
/// EntityId
///

#[derive(Debug, FromMeta)]
pub struct EntityId {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(multiple, rename = "key")]
    pub keys: Vec<Ident>,

    #[darling(default)]
    pub traits: Traits,
}

impl AsMacro for EntityId {
    fn def(&self) -> &Def {
        &self.def
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone().with_default_traits();

        traits.extend(vec![Trait::Copy, Trait::EntityIdKind, Trait::Into]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        use crate::node_traits::*;

        match t {
            Trait::Into => IntoTrait::tokens(self),

            _ => None,
        }
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Sorted => Trait::Sorted.derive_attribute(),
            _ => None,
        }
    }
}

impl AsSchema for EntityId {}

impl ToTokens for EntityId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def;
        let keys = self.keys.iter().map(quote::ToTokens::to_token_stream);

        tokens.extend(quote! {
            pub enum #ident {
                #(#keys,)*
            }
        });
    }
}
