use crate::prelude::*;

///
/// FromTrait
///

pub struct FromTrait {}

///
/// Enum
///

impl Imp<Enum> for FromTrait {
    fn strategy(node: &Enum) -> Option<TraitStrategy> {
        Some(from_type_view(node))
    }
}

///
/// EnumValue
///

impl Imp<EnumValue> for FromTrait {
    fn strategy(node: &EnumValue) -> Option<TraitStrategy> {
        Some(from_type_view(node))
    }
}

///
/// List
///

impl Imp<List> for FromTrait {
    fn strategy(node: &List) -> Option<TraitStrategy> {
        let item = &node.item.type_expr();

        let q = quote! {
            fn from(entries: Vec<I>) -> Self {
                Self(entries
                    .into_iter()
                    .map(Into::into)
                    .collect())
            }
        };

        let tokens = Implementor::new(node.def(), Trait::From)
            .set_tokens(q)
            .add_impl_constraint(quote!(I: Into<#item>))
            .add_impl_generic(quote!(I))
            .add_trait_generic(quote!(Vec<I>))
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Map
///

impl Imp<Map> for FromTrait {
    fn strategy(node: &Map) -> Option<TraitStrategy> {
        let key = &node.key.type_expr();
        let value = &node.value.type_expr();

        let q = quote! {
            fn from(entries: Vec<(IK, IV)>) -> Self {
                Self(entries
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect())
            }
        };

        let tokens = Implementor::new(node.def(), Trait::From)
            .set_tokens(q)
            .add_impl_constraint(quote!(IK: Into<#key>))
            .add_impl_constraint(quote!(IV: Into<#value>))
            .add_impl_generic(quote!(IK))
            .add_impl_generic(quote!(IV))
            .add_trait_generic(quote!(Vec<(IK, IV)>))
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Newtype
///

impl Imp<Newtype> for FromTrait {
    fn strategy(node: &Newtype) -> Option<TraitStrategy> {
        let item = &node.item.type_expr();

        let q = quote! {
            fn from(t: T) -> Self {
                Self(t.into())
            }
        };

        let tokens = Implementor::new(node.def(), Trait::From)
            .set_tokens(q)
            .add_impl_constraint(quote!(T: Into<#item>))
            .add_impl_generic(quote!(T))
            .add_trait_generic(quote!(T))
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Record
///

impl Imp<Record> for FromTrait {
    fn strategy(node: &Record) -> Option<TraitStrategy> {
        Some(from_type_view(node))
    }
}

///
/// Set
///

impl Imp<Set> for FromTrait {
    fn strategy(node: &Set) -> Option<TraitStrategy> {
        let item = &node.item.type_expr();

        let q = quote! {
            fn from(entries: Vec<I>) -> Self {
                Self(entries
                    .into_iter()
                    .map(Into::into)
                    .collect())
            }
        };

        let tokens = Implementor::new(node.def(), Trait::From)
            .set_tokens(q)
            .add_impl_constraint(quote!(I: Into<#item>))
            .add_impl_generic(quote!(I))
            .add_trait_generic(quote!(Vec<I>))
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}

///
/// Tuple
///

impl Imp<Tuple> for FromTrait {
    fn strategy(node: &Tuple) -> Option<TraitStrategy> {
        Some(from_type_view(node))
    }
}

///
/// Helpers
///

/// from_type_view
fn from_type_view(node: &impl HasViewTypes) -> TraitStrategy {
    let view_ident = node.view_ident();

    let q = quote! {
        fn from(view: #view_ident) -> Self {
            <Self as ::mimic::core::traits::TypeView>::from_view(view)
        }
    };

    let tokens = Implementor::new(node.def(), Trait::From)
        .set_tokens(q)
        .add_trait_generic(quote!(#view_ident))
        .to_token_stream();

    TraitStrategy::from_impl(tokens)
}
