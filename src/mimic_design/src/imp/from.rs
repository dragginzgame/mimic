use super::Implementor;
use crate::node::{List, MacroNode, Map, Newtype, Set, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// LIST
///

// list
pub fn list(node: &List, t: Trait) -> Option<TokenStream> {
    let item = &node.item;
    let tokens = quote! {
        fn from(items: Vec<#item>) -> Self {
            Self(items)
        }
    };

    let tokens = Implementor::new(node.def(), t)
        .add_trait_generic(quote!(Vec<#item>))
        .set_tokens(tokens)
        .to_token_stream();

    Some(tokens)
}

///
/// MAP
///

// map
pub fn map(node: &Map, t: Trait) -> Option<TokenStream> {
    let key = &node.key;
    let value = &node.value;

    let q = quote! {
        fn from(entries: Vec<(IK, IV)>) -> Self {
            Self(entries
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect())
        }
    };

    let tokens = Implementor::new(node.def(), t)
        .set_tokens(q)
        .add_impl_constraint(quote!(IK: Into<#key>))
        .add_impl_constraint(quote!(IV: Into<#value>))
        .add_impl_generic(quote!(IK))
        .add_impl_generic(quote!(IV))
        .add_trait_generic(quote!(Vec<(IK, IV)>))
        .to_token_stream();

    Some(tokens)
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
pub fn newtype(node: &Newtype, t: Trait) -> Option<TokenStream> {
    let item = &node.item;
    let primitive = &node.primitive;
    let q = quote! {
        fn from(t: T) -> Self {
            Self(<#item as std::convert::From<#primitive>>::from(t.into()))
        }
    };

    let tokens = Implementor::new(node.def(), t)
        .set_tokens(q)
        .add_impl_constraint(quote!(T: Into<#primitive>))
        .add_impl_generic(quote!(T))
        .add_trait_generic(quote!(T))
        .to_token_stream();

    Some(tokens)
}

///
/// SET
///

// set
pub fn set(node: &Set, t: Trait) -> Option<TokenStream> {
    let item = &node.item;

    let q = quote! {
        fn from(entries: Vec<I>) -> Self {
            Self(entries
                .into_iter()
                .map(Into::into)
                .collect())
        }
    };

    let tokens = Implementor::new(node.def(), t)
        .set_tokens(q)
        .add_impl_constraint(quote!(I: Into<#item>))
        .add_impl_generic(quote!(I))
        .add_trait_generic(quote!(Vec<I>))
        .to_token_stream();

    Some(tokens)
}
