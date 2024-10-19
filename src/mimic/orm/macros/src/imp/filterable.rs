use super::Implementor;
use crate::node::{MacroNode, Newtype, Trait};
use orm::types::PrimitiveGroup;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let inner =
        if let Some(PrimitiveGroup::Float | PrimitiveGroup::Integer | PrimitiveGroup::String) =
            &node.primitive.map(|p| p.group())
        {
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

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}
