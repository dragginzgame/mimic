use crate::{
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use schema::Schemable;

///
/// Validator
///

#[derive(Clone, Debug, FromMeta)]
pub struct Validator {
    #[darling(default, skip)]
    pub def: Def,

    #[darling(default)]
    pub debug: bool,
}

impl Node for Validator {
    fn expand(&self) -> TokenStream {
        let Def {
            ident, generics, ..
        } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let imp = self.imp();
        let q = quote! {
            #schema
            pub struct #ident #generics {}
            #imp
        };

        // debug
        assert!(!self.debug, "{q}");

        q
    }
}

impl MacroNode for Validator {
    fn def(&self) -> &Def {
        &self.def
    }
}

impl Schemable for Validator {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();

        quote! {
            ::mimic::schema::node::SchemaNode::Validator(::mimic::schema::node::Validator {
                def: #def,
            })
        }
    }
}

impl TraitNode for Validator {
    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
