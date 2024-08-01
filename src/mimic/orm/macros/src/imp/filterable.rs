use crate::{
    imp::Implementor,
    node::{MacroNode, Newtype, Trait},
};
use orm::types::PrimitiveGroup;
use proc_macro2::TokenStream;
use quote::ToTokens;

// newtype
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let inner = match &node.primitive.map(|p| p.group()) {
        Some(PrimitiveGroup::Float | PrimitiveGroup::Integer | PrimitiveGroup::String) => {
            quote! {
                Some(self.to_string())
            }
        }
        _ => quote!(None),
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
