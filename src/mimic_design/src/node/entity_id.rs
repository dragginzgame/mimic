use crate::{
    imp::{self, Imp},
    node::{Def, MacroNode, Node, Sorted, Trait, TraitNode, TraitTokens, Traits},
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

    #[darling(default)]
    pub sorted: Sorted,

    #[darling(multiple, rename = "key")]
    pub keys: Vec<Ident>,
}

impl Node for EntityId {
    fn expand(&self) -> TokenStream {
        let Self { sorted, .. } = self;
        let Def { ident, .. } = &self.def;
        let TraitTokens { derive, impls } = self.trait_tokens();

        // quote
        let keys = self.keys.iter().map(quote::ToTokens::to_token_stream);
        let q = quote! {
            #derive
            #sorted
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
        let mut traits = Traits::default();
        traits.extend(vec![
            Trait::Copy,
            Trait::Display,
            Trait::EntityId,
            Trait::Into,
        ]);

        traits.list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        match t {
            Trait::Into => imp::IntoTrait::tokens(self, t),

            _ => imp::any(self, t),
        }
    }
}
