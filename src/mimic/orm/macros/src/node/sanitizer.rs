use crate::{
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use orm_schema::Schemable;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Sanitizer
///

#[derive(Clone, Debug, FromMeta)]
pub struct Sanitizer {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub debug: bool,
}

impl Node for Sanitizer {
    fn expand(&self) -> TokenStream {
        let Def { ident, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let imp = &self.imp();
        let q = quote! {
            #schema
            pub struct #ident {}
            #imp
        };

        // debug
        if self.debug {
            let s = q.to_string();
            return quote!(compile_error!(#s));
        }

        q
    }
}

impl MacroNode for Sanitizer {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Sanitizer {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Sanitizer(::mimic::orm::schema::node::Sanitizer {
                def: #def,
            })
        }
    }
}

impl TraitNode for Sanitizer {
    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
