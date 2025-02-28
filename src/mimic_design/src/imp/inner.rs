use super::Implementor;
use crate::node::{MacroNode, Newtype, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
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

    Implementor::new(node.def(), t)
        .add_trait_generic(quote!(#primitive))
        .set_tokens(q)
        .to_token_stream()
}
