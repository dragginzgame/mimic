use crate::{
    imp::{Imp, Implementor},
    node::{Newtype, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// QueryValueTrait
///

pub struct QueryValueTrait {}

///
/// Newtype
///

impl Imp<Newtype> for FormaQueryValueTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let q = quote! {
            fn to_query_value(&self) -> Option<String> {
                self.0.to_query_value()
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
