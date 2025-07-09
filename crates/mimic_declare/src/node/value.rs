use crate::{
    node::Item,
    traits::{AsSchema, AsType},
};
use darling::FromMeta;
use mimic_schema::types::Cardinality;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// Value
///

#[derive(Clone, Debug, FromMeta)]
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

impl AsSchema for Value {
    fn schema(&self) -> TokenStream {
        let cardinality = &self.cardinality();
        let item = &self.item.schema();

        quote!(
            ::mimic::schema::node::Value {
                cardinality: #cardinality,
                item: #item,
            }
        )
    }
}

impl AsType for Value {
    fn as_type(&self) -> TokenStream {
        let item = &self.item;

        match self.cardinality() {
            Cardinality::One => quote!(#item),
            Cardinality::Opt => quote!(Option<#item>),
            Cardinality::Many => quote!(Vec<#item>),
        }
    }

    fn as_view_type(&self) -> TokenStream {
        let item = &self.item;
        let item_view = AsType::as_view_type(item);

        match self.cardinality() {
            Cardinality::One => quote!(#item_view),
            Cardinality::Opt => quote!(Option<#item_view>),
            Cardinality::Many => quote!(Vec<#item_view>),
        }
    }
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.as_type());
    }
}
