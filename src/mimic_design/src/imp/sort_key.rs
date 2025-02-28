use super::Implementor;
use crate::node::{Newtype, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

// newtype
// simply delegates to the wrapped type
pub fn newtype(node: &Newtype, t: Trait) -> Option<TokenStream> {
    let q = quote! {
        fn format(&self) -> String {
            self.0.format()
        }
    };

    let tokens = Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}
