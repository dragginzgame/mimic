use super::Implementor;
use crate::node::{MacroNode, Selector, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// selector
pub fn selector(node: &Selector, t: Trait) -> TokenStream {
    let mut inner = quote!();

    // iterate variants
    for variant in &node.variants {
        let name = &variant.name;
        let value = &variant.value;

        inner.extend(quote! {
            Self::#name => #value,
        });
    }

    // quote
    let q = quote! {
        fn value(&self) -> i32 {
            match self {
                #inner
            }
        }
    };

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}
