use super::{Def, MacroNode, Node, Sorted, Trait, TraitNode, Traits};
use crate::imp;
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

        // quote
        let derive = self.derive();
        let keys = self.keys.iter().map(quote::ToTokens::to_token_stream);
        let imp = self.imp();
        let q = quote! {
            #derive
            #sorted
            pub enum #ident {
                #(#keys,)*
            }
            #imp
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

    fn map_imp(&self, t: Trait) -> TokenStream {
        match t {
            Trait::Into => imp::into::entity_id(self, t),

            _ => imp::any(self, t),
        }
    }
}
