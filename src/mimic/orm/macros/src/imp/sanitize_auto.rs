use super::{Implementor, Trait};
use crate::node::Newtype;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let mut rules = quote!();

    // sanitizers
    for san in &node.sanitizers {
        let path = &san.path;
        let args = &san.args;
        let quote_rule = quote! {
            self.0 = #path::sanitize(&self.0, #(#args),*).into();
        };

        rules.extend(quote_rule);
    }

    // quote
    let q = quote! {
        fn sanitize_auto(&mut self) {
            #rules
        }
    };

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}
