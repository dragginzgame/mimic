use crate::{
    node::Def,
    node_traits::{self, Trait, Traits},
    traits::{Macro, Schemable},
};
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Canister
/// regardless of the path, the name is used to uniquely identify each canister
///

#[derive(Debug, FromMeta)]
pub struct Canister {
    #[darling(skip, default)]
    pub def: Def,
}

impl Macro for Canister {
    fn def(&self) -> &Def {
        &self.def
    }

    fn traits(&self) -> Vec<Trait> {
        Traits::default().list()
    }

    fn map_trait(&self, t: Trait) -> Option<TokenStream> {
        node_traits::any(self, t)
    }
}

impl Schemable for Canister {
    fn schema(&self) -> TokenStream {
        let def = self.def.schema();

        quote! {
            ::mimic::schema::node::Schemable::Canister(::mimic::schema::node::Canister{
                def: #def,
            })
        }
    }
}
