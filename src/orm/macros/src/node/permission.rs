use crate::{
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use schema::Schemable;

///
/// Permission
///

#[derive(Clone, Debug, FromMeta)]
pub struct Permission {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub debug: bool,
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
        assert!(!self.debug, "{q}");

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
            ::mimic::schema::node::SchemaNode::Permission(::mimic::schema::node::Permission {
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
