use super::Implementor;
use crate::node::{Newtype, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// newtype
// simply delegates to the wrapped type
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let q = quote! {
        fn on_create(&self) -> Self {
            Self(self.0)
        }

        fn format(&self) -> String {
            self.0.format()
        }
    };

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}
