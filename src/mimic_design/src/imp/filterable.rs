use super::Implementor;
use crate::node::{MacroNode, Newtype, PrimitiveGroup, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> Option<TokenStream> {
    let inner = if matches!(
        node.primitive.group(),
        PrimitiveGroup::Float
            | PrimitiveGroup::Integer
            | PrimitiveGroup::String
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

    let tokens = Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}
