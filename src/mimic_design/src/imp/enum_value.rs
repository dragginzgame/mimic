use super::Implementor;
use crate::node::{EnumValue, MacroNode, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// ENUM_VALUE
///

// enum_value
pub fn enum_value(node: &EnumValue, t: Trait) -> Option<TokenStream> {
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

    let tokens = Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}
