use crate::{
    imp::{Imp, Implementor},
    node::{Newtype, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// FormatSortKeyTrait
///

pub struct FormatSortKeyTrait {}

///
/// Newtype
///

impl Imp<Newtype> for FormatSortKeyTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let q = quote! {
            fn to_sort_key_part(&self) -> Option<String> {
                self.0.to_sort_key_part()
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
