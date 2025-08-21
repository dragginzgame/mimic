use crate::{
    node::Item,
    traits::{HasSchemaPart, HasTypePart},
};
use darling::FromMeta;
use mimic_schema::types::Cardinality;
use proc_macro2::TokenStream;
use quote::quote;

///
/// Value
///

#[derive(Clone, Debug, Default, FromMeta)]
pub struct Value {
    #[darling(default)]
    pub opt: bool,

    #[darling(default)]
    pub many: bool,

    pub item: Item,
}

impl Value {
    // cardinality
    pub fn cardinality(&self) -> Cardinality {
        match (&self.opt, &self.many) {
            (false, false) => Cardinality::One,
            (true, false) => Cardinality::Opt,
            (false, true) => Cardinality::Many,
            (true, true) => panic!("cardinality cannot be opt and many"),
        }
    }
}

impl HasSchemaPart for Value {
    fn schema_part(&self) -> TokenStream {
        let cardinality = &self.cardinality();
        let item = &self.item.schema_part();

        quote!(
            ::mimic::schema::node::Value {
                cardinality: #cardinality,
                item: #item,
            }
        )
    }
}

impl HasTypePart for Value {
    fn type_part(&self) -> TokenStream {
        let item = &self.item.type_part();

        match self.cardinality() {
            Cardinality::One => quote!(#item),
            Cardinality::Opt => quote!(Option<#item>),
            Cardinality::Many => quote!(Vec<#item>),
        }
    }

    fn view_type_part(&self) -> TokenStream {
        let item_view = &self.item.view_type_part();

        match self.cardinality() {
            Cardinality::One => quote!(#item_view),
            Cardinality::Opt => quote!(Option<#item_view>),
            Cardinality::Many => quote!(Vec<#item_view>),
        }
    }
}
