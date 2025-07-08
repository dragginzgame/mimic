use crate::{
    node::EnumValue,
    node_traits::{Imp, Implementor, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// EnumValueKindTrait
///

pub struct EnumValueKindTrait {}

///
/// EnumValue
///

impl Imp<EnumValue> for EnumValueKindTrait {
    fn tokens(node: &EnumValue) -> Option<TokenStream> {
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

        let tokens = Implementor::new(&node.def, Trait::EnumValueKind)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
