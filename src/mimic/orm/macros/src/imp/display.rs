use crate::{
    imp::Implementor,
    node::{MacroNode, Newtype, Trait},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// newtype
// simply delegates to the wrapped type
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let q = quote! {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            write!(f, "{}", self.0)
        }
    };

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}
