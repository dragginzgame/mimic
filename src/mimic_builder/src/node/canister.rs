use crate::{
    helper::{quote_one, to_string},
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Canister
/// regardless of the path, the name is used to uniquely identify each canister
///

#[derive(Debug, FromMeta)]
pub struct Canister {
    #[darling(skip, default)]
    pub def: Def,

    pub name: String,
}

impl Node for Canister {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def();

        // quote
        let schema = self.ctor_schema();
        let imp = &self.imp();
        let q = quote! {
            #schema
            pub struct #ident {}
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

impl MacroNode for Canister {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Canister {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let name = quote_one(&self.name, to_string);

        quote! {
            ::mimic::schema::node::SchemaNode::Canister(::mimic::schema::node::Canister{
                def: #def,
                name: #name,
            })
        }
    }
}

impl TraitNode for Canister {
    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
