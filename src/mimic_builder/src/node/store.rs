use crate::{
    helper::{quote_one, to_path, to_string},
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

///
/// Store
///

#[derive(Debug, FromMeta)]
pub struct Store {
    #[darling(default, skip)]
    pub def: Def,

    pub ident: Ident,
    pub canister: Path,
    pub memory_id: u8,
}

impl Node for Store {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let imp = self.imp();
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

impl MacroNode for Store {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Store {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let ident = quote_one(&self.ident, to_string);
        let canister = quote_one(&self.canister, to_path);
        let memory_id = &self.memory_id;

        quote! {
            ::mimic::schema::node::SchemaNode::Store(::mimic::schema::node::Store{
                def: #def,
                ident: #ident,
                canister: #canister,
                memory_id: #memory_id,
            })
        }
    }
}

impl TraitNode for Store {
    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
