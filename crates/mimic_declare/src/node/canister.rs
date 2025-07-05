use crate::{
    node::Def,
    node_traits::{self, Trait, Traits},
    traits::{MacroNode, SchemaNode},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Canister
/// regardless of the path, the name is used to uniquely identify each canister
///

#[derive(Debug, FromMeta)]
pub struct Canister {
    #[darling(skip, default)]
    pub def: Def,
}

impl ToTokens for Canister {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Def { ident, .. } = &self.def();

        tokens.extend(quote! {
            pub struct #ident {}
        });
    }
}

impl MacroNode for Canister {
    fn def(&self) -> &Def {
        &self.def
    }

    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        node_traits::any(self, t)
    }
}

impl SchemaNode for Canister {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Canister(::mimic::schema::node::Canister{
                def: #def,
            })
        }
    }
}
