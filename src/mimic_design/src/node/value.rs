use crate::{node::Item, traits::Schemable};
use darling::FromMeta;
use mimic_common::types::Cardinality;
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

impl Schemable for Value {
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

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let item = &self.item;

        let q = match self.cardinality() {
            Cardinality::One => quote!(#item),
            Cardinality::Opt => quote!(Option<#item>),
            Cardinality::Many => {
                if item.is_relation() {
                    quote!(::mimic::base::types::RelationSet)
                } else {
                    quote!(Vec<#item>)
                }
            }
        };

        tokens.extend(q);
    }
}
