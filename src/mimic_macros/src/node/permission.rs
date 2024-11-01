use crate::{
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Permission
///

#[derive(Clone, Debug, FromMeta)]
pub struct Permission {
    #[darling(default, skip)]
    pub def: Def,
}

impl Node for Permission {
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

impl MacroNode for Permission {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Permission {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();

        quote! {
            ::mimic::orm::schema::node::SchemaNode::Permission(::mimic::orm::schema::node::Permission {
                def: #def,
            })
        }
    }
}

impl TraitNode for Permission {
    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
