use super::Implementor;
use crate::node::{Enum, MacroNode, Newtype, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// enum_
pub fn enum_(node: &Enum, t: Trait) -> TokenStream {
    let q = if node.is_orderable() {
        quote! {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                Ord::cmp(self, other)
            }
        }
    } else {
        quote!()
    };

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let q = match &node.primitive {
        Some(primitive) if primitive.is_orderable() => {
            quote! {
                fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                    Ord::cmp(self, other)
                }
            }
        }
        _ => quote!(),
    };

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}
