use super::Implementor;
use crate::node::{List, MacroNode, Map, Newtype, Trait};
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

///
/// NEWTYPE
///

//
// newtype
//
// possibility to optimise here as we do have fine-grained control over the
// From implementation for each PrimitiveType
//
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let item = &node.item;
    let primitive = &node.primitive;

    let mut imp = Implementor::new(node.def(), t);
    imp = imp.add_trait_generic(quote!(T));

    let tokens = quote! {
        fn from(t: T) -> Self {
            Self(<#item as std::convert::From<#primitive>>::from(t.into()))
        }
    };

    imp.set_tokens(tokens)
        .add_impl_constraint(quote!(T: Into<#primitive>))
        .add_impl_generic(quote!(T))
        .to_token_stream()
}
