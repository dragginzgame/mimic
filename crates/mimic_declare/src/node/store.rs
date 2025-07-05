use crate::{
    helper::{quote_one, to_path, to_str_lit},
    node::Def,
    node_traits::{self, Trait, Traits},
    traits::{Macro, Schemable},
};
use darling::FromMeta;
use mimic_schema::types::StoreType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

///
/// Store
///

#[derive(Debug, FromMeta)]
pub struct Store {
    #[darling(default, skip)]
    pub def: Def,

    pub ident: Ident,
    pub ty: StoreType,
    pub canister: Path,
    pub memory_id: u8,
}

impl Macro for Store {
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

impl Schemable for Store {
    fn schema(&self) -> TokenStream {
        let def = &self.def.schema();
        let ident = quote_one(&self.ident, to_str_lit);
        let ty = &self.ty;
        let canister = quote_one(&self.canister, to_path);
        let memory_id = &self.memory_id;

        quote! {
            ::mimic::schema::node::Schemable::Store(::mimic::schema::node::Store{
                def: #def,
                ident: #ident,
                ty: #ty,
                canister: #canister,
                memory_id: #memory_id,
            })
        }
    }
}
