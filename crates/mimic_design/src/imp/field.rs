use crate::{
    imp::{Imp, Implementor},
    node::Newtype,
    traits::Trait,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// FieldValueTrait
///

pub struct FieldValueTrait {}

///
/// Newtype
///

impl Imp<Newtype> for FieldValueTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let q = quote! {
            fn to_value(&self) -> ::mimic::core::value::Value {
                self.0.to_value()
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
