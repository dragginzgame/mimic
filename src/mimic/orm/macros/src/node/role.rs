use crate::imp;
use crate::{
    helper::{quote_option, quote_vec, to_path},
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use orm_schema::traits::Schemable;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

///
/// Role
///

#[derive(Clone, Debug, FromMeta)]
pub struct Role {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub parent: Option<Path>,

    #[darling(multiple, rename = "permission")]
    pub permissions: Vec<Path>,
}

impl Node for Role {
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

impl MacroNode for Role {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Role {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();
        let parent = quote_option(&self.parent, to_path);
        let permissions = quote_vec(&self.permissions, to_path);

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Role(::mimic::orm::schema::node::Role {
                def: #def,
                parent: #parent,
                permissions: #permissions,
            })
        }
    }
}

impl TraitNode for Role {
    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
