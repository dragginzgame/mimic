use crate::{
    imp,
    node::{Def, MacroNode, Node, Trait, TraitNode, Traits},
    traits::Schemable,
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Validator
///

#[derive(Clone, Debug, FromMeta)]
pub struct Validator {
    #[darling(default, skip)]
    pub def: Def,
}

impl Node for Validator {
    fn expand(&self) -> TokenStream {
        let Def { tokens, .. } = &self.def;

        // quote
        let schema = self.ctor_schema();
        let derive = self.derive();
        let imp = self.imp();
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
        let mut traits = Traits::default().list();
        traits.push(Trait::Default);

        traits
    }

    fn map_imp(&self, t: Trait) -> TokenStream {
        imp::any(self, t)
    }
}
