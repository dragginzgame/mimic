use crate::{
    imp::Implementor,
    node::{EnumValue, MacroNode, Trait},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

///
/// ENUM_VALUE
///

// enum_value
pub fn enum_value(node: &EnumValue, t: Trait) -> TokenStream {
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
        fn value(&self) -> u32 {
            match self {
                #inner
            }
        }
    };

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}
