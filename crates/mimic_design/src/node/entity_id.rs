use crate::{
    node::{Def, MacroNode, Node, TraitNode, TraitTokens},
    traits::{self, Trait, Traits},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
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

impl Node for EntityId {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;
        let TraitTokens { derive, impls } = self.trait_tokens();

        // quote
        let keys = self.keys.iter().map(quote::ToTokens::to_token_stream);
        let q = quote! {
            #derive
            pub enum #ident {
                #(#keys,)*
            }
            #impls
        };

        // debug
        if self.def.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s););
        }

        q
    }
}

impl MacroNode for EntityId {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl TraitNode for EntityId {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = self.traits.clone();
        traits.extend(vec![Trait::Copy, Trait::EntityIdKind]);

        traits.list()
    }

    fn custom_impl(&self) -> Option<TokenStream> {
        crate::node::imp::entity_id::tokens(self)
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        traits::any(self, t)
    }

    fn map_attribute(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Sorted => Trait::Sorted.derive_attribute(),
            _ => None,
        }
    }
}
