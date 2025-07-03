use crate::{
    node::{Def, MacroNode, Node, TraitNode, TraitTokens},
    schema::Schemable,
    traits::{self, Trait, Traits},
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
        let TraitTokens { derive, impls } = self.trait_tokens();

        // quote
        let schema = self.ctor_schema();
        let q = quote! {
            #schema
            #derive
            #tokens
            #impls
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

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        traits::any(self, t)
    }
}
