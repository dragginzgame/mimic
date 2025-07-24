use crate::{
    node::{Entity, Enum, EnumValue, List, Map, Newtype, Record, Set, Tuple},
    node_traits::{Imp, Implementor, Trait},
    traits::{HasIdent, HasType, HasTypePart},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// FromTrait
///

pub struct FromTrait {}

///
/// Entity
///

impl Imp<Entity> for FromTrait {
    fn tokens(node: &Entity) -> Option<TokenStream> {
        from_type_view(node)
    }
}

///
/// Enum
///

impl Imp<Enum> for FromTrait {
    fn tokens(node: &Enum) -> Option<TokenStream> {
        from_type_view(node)
    }
}

///
/// EnumValue
///

impl Imp<EnumValue> for FromTrait {
    fn tokens(node: &EnumValue) -> Option<TokenStream> {
        from_type_view(node)
    }
}

///
/// List
///

impl Imp<List> for FromTrait {
    fn tokens(node: &List) -> Option<TokenStream> {
        let item = &node.item.type_part();

        let q = quote! {
            fn from(entries: Vec<I>) -> Self {
                Self(entries
                    .into_iter()
                    .map(Into::into)
                    .collect())
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::From)
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
    fn tokens(node: &Map) -> Option<TokenStream> {
        let key = &node.key.type_part();
        let value = &node.value.type_part();

        let q = quote! {
            fn from(entries: Vec<(IK, IV)>) -> Self {
                Self(entries
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect())
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::From)
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
    fn tokens(node: &Newtype) -> Option<TokenStream> {
        let item = &node.item.type_part();

        let q = quote! {
            fn from(t: T) -> Self {
                Self(t.into())
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::From)
            .set_tokens(q)
            .add_impl_constraint(quote!(T: Into<#item>))
            .add_impl_generic(quote!(T))
            .add_trait_generic(quote!(T))
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Record
///

impl Imp<Record> for FromTrait {
    fn tokens(node: &Record) -> Option<TokenStream> {
        from_type_view(node)
    }
}

///
/// Set
///

impl Imp<Set> for FromTrait {
    fn tokens(node: &Set) -> Option<TokenStream> {
        let item = &node.item.type_part();

        let q = quote! {
            fn from(entries: Vec<I>) -> Self {
                Self(entries
                    .into_iter()
                    .map(Into::into)
                    .collect())
            }
        };

        let tokens = Implementor::new(node.ident(), Trait::From)
            .set_tokens(q)
            .add_impl_constraint(quote!(I: Into<#item>))
            .add_impl_generic(quote!(I))
            .add_trait_generic(quote!(Vec<I>))
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Tuple
///

impl Imp<Tuple> for FromTrait {
    fn tokens(node: &Tuple) -> Option<TokenStream> {
        from_type_view(node)
    }
}

/// from_type_view
#[allow(clippy::unnecessary_wraps)]
fn from_type_view(m: &impl HasType) -> Option<TokenStream> {
    let ident = m.ident();
    let view_ident = m.view_ident();

    let q = quote! {
        fn from(view: #view_ident) -> Self {
            <#ident as ::mimic::core::traits::TypeView>::from_view(view)
        }
    };

    let tokens = Implementor::new(ident, Trait::From)
        .set_tokens(q)
        .add_trait_generic(quote!(#view_ident))
        .to_token_stream();

    Some(tokens)
}
