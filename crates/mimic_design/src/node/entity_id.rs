use crate::{
    node::Def,
    node_traits::{self, Trait, Traits},
    traits::MacroNode,
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

impl MacroNode for EntityId {
    fn def(&self) -> &Def {
        &self.def
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.extend(vec![Trait::Copy, Trait::EntityIdKind]);

        traits.list()
    }

    fn custom_impl(&self) -> Option<TokenStream> {
        crate::node::imp::entity_id::tokens(self)
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        node_traits::any(self, t)
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Sorted => Trait::Sorted.derive_attribute(),
            _ => None,
        }
    }
}
