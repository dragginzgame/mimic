use super::Implementor;
use crate::node::{Newtype, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> Option<TokenStream> {
    let primitive = &node.primitive;

    // quote
    let q = quote! {
        fn inner(&self) -> &#primitive {
            self.0.inner()
        }

        fn into_inner(self) -> #primitive {
            self.0.into_inner()
        }
    };

    let tokens = Implementor::new(&node.def, t)
        .add_trait_generic(quote!(#primitive))
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}
