use super::Implementor;
use crate::node::{Selector, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// selector
// any selector type can act as a validator
pub fn selector(node: &Selector, t: Trait) -> TokenStream {
    // quote
    let q = quote! {
        fn validate_string(&self) -> ::std::result::Result<(), ::mimic::orm::types::Error> {
            Ok(())
        }
    };

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}
