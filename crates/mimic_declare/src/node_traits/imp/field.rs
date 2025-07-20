use crate::{
    node::Newtype,
    node_traits::{Imp, Implementor, Trait},
    traits::AsMacro,
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
    fn tokens(node: &Newtype) -> Option<TokenStream> {
        let q = quote! {
            fn to_value(&self) -> ::mimic::core::value::Value {
                self.0.to_value()
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::FieldValue)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
