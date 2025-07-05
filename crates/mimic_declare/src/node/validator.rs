use crate::{
    node::Def,
    node_traits::{self, Trait, Traits},
    traits::{Macro, Schemable},
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

impl Macro for Validator {
    fn def(&self) -> &Def {
        &self.def
    }

    fn traits(&self) -> Vec<Trait> {
        let mut traits = Traits::default().list();
        traits.push(Trait::Default);

        traits
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        node_traits::any(self, t)
    }
}

impl Schemable for Validator {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();

        quote! {
            ::mimic::schema::node::Schemable::Validator(::mimic::schema::node::Validator {
                def: #def,
            })
        }
    }
}
