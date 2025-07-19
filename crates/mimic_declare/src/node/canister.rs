use crate::{
    node::Def,
    node_traits::{Trait, Traits},
    traits::{AsMacro, AsSchema, AsType, MacroEmitter, SchemaKind},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

///
/// Canister
/// regardless of the path, the name is used to uniquely identify each canister
///

#[derive(Debug, FromMeta)]
pub struct Canister {
    #[darling(skip, default)]
    pub def: Def,
}

impl AsMacro for Canister {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }

    fn traits(&self) -> Vec<Trait> {
        Traits::default().with_path_trait().list()
    }
}

impl AsSchema for Canister {
    const KIND: SchemaKind = SchemaKind::Full;

    fn schema(&self) -> TokenStream {
        let def = self.def.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Canister(::mimic::schema::node::Canister{
                def: #def,
            })
        }
    }
}

impl AsType for Canister {
    fn as_type(&self) -> Option<TokenStream> {
        let ident = self.ident();

        Some(quote! {
            pub struct #ident {}
        })
    }
}

impl ToTokens for Canister {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens())
    }
}
