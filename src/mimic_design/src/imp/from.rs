use crate::{
    imp::{Imp, Implementor},
    node::{List, Map, Newtype, Set, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// FromTrait
///

pub struct FromTrait {}

///
/// List
///

impl Imp<List> for FromTrait {
    fn tokens(node: &List, t: Trait) -> Option<TokenStream> {
        let item = &node.item;

        let q = quote! {
            fn from(entries: Vec<I>) -> Self {
                Self(entries
                    .into_iter()
                    .map(Into::into)
                    .collect())
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .add_impl_constraint(quote!(I: Into<#item>))
            .add_impl_generic(quote!(I))
            .add_trait_generic(quote!(Vec<I>))
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Map
///

impl Imp<Map> for FromTrait {
    fn tokens(node: &Map, t: Trait) -> Option<TokenStream> {
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

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .add_impl_constraint(quote!(IK: Into<#key>))
            .add_impl_constraint(quote!(IV: Into<#value>))
            .add_impl_generic(quote!(IK))
            .add_impl_generic(quote!(IV))
            .add_trait_generic(quote!(Vec<(IK, IV)>))
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Newtype
///

impl Imp<Newtype> for FromTrait {
    fn tokens(node: &Newtype, t: Trait) -> Option<TokenStream> {
        let item = &node.item;
        let primitive = &node.primitive;

        let q = quote! {
            fn from(t: T) -> Self {
                Self(<#item as std::convert::From<#primitive>>::from(t.into()))
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .add_impl_constraint(quote!(T: Into<#primitive>))
            .add_impl_generic(quote!(T))
            .add_trait_generic(quote!(T))
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Set
///

impl Imp<Set> for FromTrait {
    fn tokens(node: &Set, t: Trait) -> Option<TokenStream> {
        let item = &node.item;

        let q = quote! {
            fn from(entries: Vec<I>) -> Self {
                Self(entries
                    .into_iter()
                    .map(Into::into)
                    .collect())
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .add_impl_constraint(quote!(I: Into<#item>))
            .add_impl_generic(quote!(I))
            .add_trait_generic(quote!(Vec<I>))
            .to_token_stream();

        Some(tokens)
    }
}
