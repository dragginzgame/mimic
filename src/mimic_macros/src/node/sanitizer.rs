use crate::{
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Sanitizer
///

#[derive(Clone, Debug, FromMeta)]
pub struct Sanitizer {
    #[darling(default, skip)]
    pub def: Def,
}

impl Node for Sanitizer {
    fn expand(&self) -> TokenStream {
        let Def { tokens, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let derive = self.derive();
        let imp = &self.imp();
        let q = quote! {
            #schema
            #derive
            #tokens
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
        let mut traits = Traits::default().list();
        traits.push(Trait::Default);

        traits
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
