use crate::{
    imp::{Imp, Implementor},
    node::{Newtype, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// FieldSortKeyTrait
///

pub struct FieldSortKeyTrait {}

///
/// Newtype
///

impl Imp<Newtype> for FieldSortKeyTrait {
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
