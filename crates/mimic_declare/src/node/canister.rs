use crate::{
    node::Def,
    schema_traits::{Trait, Traits},
    traits::{
        HasIdent, HasMacro, HasSchema, HasSchemaPart, HasTraits, HasTypePart, SchemaNodeKind,
    },
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

impl HasIdent for Canister {
    fn ident(&self) -> Ident {
        self.def.ident.clone()
    }
}

impl HasSchema for Canister {
    fn schema_node_kind() -> SchemaNodeKind {
        SchemaNodeKind::Canister
    }
}

impl HasSchemaPart for Canister {
    fn schema_part(&self) -> TokenStream {
        let def = self.def.schema_part();

        quote! {
            ::mimic::schema::node::Canister{
                def: #def,
            }
        }
    }
}

impl HasTraits for Canister {
    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default().with_path_trait();
        traits.add(Trait::CanisterKind);

        traits.list()
    }
}

impl HasTypePart for Canister {
    fn type_part(&self) -> TokenStream {
        let ident = self.ident();

        quote! {
            pub struct #ident {}
        }
    }
}

impl ToTokens for Canister {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.all_tokens());
    }
}
