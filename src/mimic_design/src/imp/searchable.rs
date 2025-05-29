use crate::{
    imp::{Imp, Implementor},
    node::{Newtype, PrimitiveGroup, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// SearchableTrait
///

pub struct SearchableTrait {}

///
/// Newtype
///

impl Imp<Newtype> for SearchableTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let inner = if matches!(
            node.primitive.group(),
            PrimitiveGroup::Float
                | PrimitiveGroup::Integer
                | PrimitiveGroup::Text
                | PrimitiveGroup::Ulid,
        ) {
            quote!(Some(self.to_string()))
        } else {
            quote!(None)
        };

        // quote
        let q = quote! {
            fn as_text(&self) -> Option<String> {
                #inner
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
