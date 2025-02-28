use super::Implementor;
use crate::node::{List, MacroNode, Map, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// LIST
///

// list
pub fn list(node: &List, t: Trait) -> TokenStream {
    let mut imp = Implementor::new(node.def(), t);

    // match cardinality
    let item = &node.item;
    imp = imp.add_trait_generic(quote!(Vec<#item>));

    let tokens = quote! {
        fn from(items: Vec<#item>) -> Self {
            Self(items)
        }
    };

    imp.set_tokens(tokens).to_token_stream()
}

///
/// MAP
///

// map
pub fn map(node: &Map, t: Trait) -> TokenStream {
    let mut imp = Implementor::new(node.def(), t);

    // match cardinality
    let item = &node.item;
    imp = imp.add_trait_generic(quote!(Vec<#item>));

    let tokens = quote! {
        fn from(items: Vec<#item>) -> Self {
            Self(items)
        }
    };

    imp.set_tokens(tokens).to_token_stream()
}
