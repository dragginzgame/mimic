use super::Implementor;
use crate::node::{Enum, Newtype, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

// enum_
pub fn enum_(node: &Enum, t: Trait) -> Option<TokenStream> {
    let q = if node.is_orderable() {
        quote! {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                Ord::cmp(self, other)
            }
        }
    } else {
        quote!()
    };

    let tokens = Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> Option<TokenStream> {
    let q = if node.primitive.is_orderable() {
        quote! {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                Ord::cmp(self, other)
            }
        }
    } else {
        quote!()
    };

    let tokens = Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}
