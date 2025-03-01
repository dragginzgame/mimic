use crate::{
    imp::{Imp, Implementor},
    node::{Newtype, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// SortKeyTrait
///

pub struct SortKeyTrait {}

///
/// Newtype
/// simply delegates to the wrapped type
///

impl Imp<Newtype> for SortKeyTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
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
}
