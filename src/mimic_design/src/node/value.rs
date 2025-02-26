use crate::{
    helper::quote_option,
    node::{Arg, Cardinality, Item},
    traits::Schemable,
};
use darling::FromMeta;
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

    #[darling(default)]
    pub default: Option<Arg>,
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
        let cardinality = &self.cardinality().schema();
        let item = &self.item.schema();
        let default = quote_option(self.default.as_ref(), Arg::schema);

        quote!(
            ::mimic::schema::node::Value {
                cardinality: #cardinality,
                item: #item,
                default: #default,
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
                    quote!(UlidSet)
                } else {
                    quote!(Vec<#item>)
                }
            }
        };

        tokens.extend(q);
    }
}
