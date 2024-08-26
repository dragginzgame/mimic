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

        inner.extend(match value {
            Some(value) => quote! ( Self::#name => Some(#value), ),
            None => quote!( Self::#name => None, ),
        });
    }

    // quote
    let q = quote! {
        fn value(&self) -> Option<i64> {
            match self {
                #inner
            }
        }
    };

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}
