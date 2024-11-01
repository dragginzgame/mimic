use super::Implementor;
use crate::node::{Cardinality, MacroNode, Map, Newtype, PrimitiveGroup, Trait, Tuple};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

///
/// Map
///

pub fn map(node: &Map, t: Trait) -> TokenStream {
    let key = &node.key;
    let value = &node.value;

    let q = quote! {
        fn from(t: Vec<(#key, #value)>) -> Self {
            let map: ::mimic::orm::types::StrHashMap<#key, #value> = t.into_iter().collect();

            Self(map)
        }
    };

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .add_trait_generic(quote!(Vec<(#key, #value)>))
        .to_token_stream()
}

///
/// Newtype
///

// newtype
//
// possibility to optimise here as we do have fine-grained control over the
// From implementation for each PrimitiveType
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let mut q = quote!();

    match node.primitive.map(|p| p.group()) {
        Some(PrimitiveGroup::Bool | PrimitiveGroup::Float) | None => {
            q.extend(newtype_inner(node, t));
        }
        Some(PrimitiveGroup::Blob | PrimitiveGroup::Integer | PrimitiveGroup::Decimal) => {
            q.extend(newtype_into_inner(node, t));
        }
        Some(PrimitiveGroup::String) => {
            q.extend(newtype_inner(node, t));
            q.extend(newtype_str(node, t));
        }
    }

    q
}

// newtype_inner
pub fn newtype_inner(node: &Newtype, t: Trait) -> TokenStream {
    let mut imp = Implementor::new(node.def(), t);

    // match cardinality
    let item = &node.value.item;
    let tokens = match node.value.cardinality() {
        Cardinality::One => {
            imp = imp.add_trait_generic(quote!(#item));

            quote! {
                fn from(item: #item) -> Self {
                    Self(item)
                }
            }
        }
        Cardinality::Opt => {
            imp = imp.add_trait_generic(quote!(#item));

            quote! {
                fn from(item: #item) -> Self {
                    Self(Some(item))
                }
            }
        }
        Cardinality::Many => {
            imp = imp.add_trait_generic(quote!(Vec<#item>));

            quote! {
                fn from(items: Vec<#item>) -> Self {
                    Self(items)
                }
            }
        }
    };

    imp.set_tokens(tokens).to_token_stream()
}

// newtype_into_inner
pub fn newtype_into_inner(node: &Newtype, t: Trait) -> TokenStream {
    let value = &node.value;
    let item = &value.item;

    let mut imp = Implementor::new(node.def(), t);

    let tokens = match value.cardinality() {
        Cardinality::One => {
            imp = imp.add_trait_generic(quote!(T));

            quote! {
                fn from(t: T) -> Self {
                    Self(t.into())
                }
            }
        }
        Cardinality::Opt => {
            imp = imp.add_trait_generic(quote!(T));

            quote! {
                fn from(t: T) -> Self {
                    Self(Some(t.into()))
                }
            }
        }
        Cardinality::Many => {
            imp = imp.add_trait_generic(quote!(Vec<T>));

            quote! {
                fn from(t: Vec<T>) -> Self {
                    let vec: Vec<#item> = t.into_iter().map(|item| item.into()).collect();
                    Self(vec)
                }
            }
        }
    };

    imp.set_tokens(tokens)
        .add_impl_constraint(quote!(T: Into<#item>))
        .add_impl_generic(quote!(T))
        .to_token_stream()
}

// newtype_str
pub fn newtype_str(node: &Newtype, t: Trait) -> TokenStream {
    let mut imp = Implementor::new(node.def(), t);

    // match cardinality
    let tokens = match node.value.cardinality() {
        Cardinality::One => {
            imp = imp.add_trait_generic(quote!(&str));

            quote! {
                fn from(s: &str) -> Self {
                    Self(s.into())
                }
            }
        }
        Cardinality::Opt => {
            imp = imp.add_trait_generic(quote!(&str));

            quote! {
                fn from(s: &str) -> Self {
                    Self(Some(s.into()))
                }
            }
        }
        Cardinality::Many => {
            imp = imp.add_trait_generic(quote!(&str));

            quote! {
                fn from(ss: Vec<&str>) -> Self {
                    let vec: Vec<&str> = ss.into_iter().map(|item| item.into()).collect();
                    Self(vec)
                }
            }
        }
    };

    imp.set_tokens(tokens).to_token_stream()
}

///
/// Tuple
///

pub fn tuple(node: &Tuple, t: Trait) -> TokenStream {
    let inner = quote!(#node);

    let q = quote! {
        fn from(t: T) -> Self {
            Self(t.into())
        }
    };

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .add_impl_constraint(quote!(T: Into<#inner>))
        .add_impl_generic(quote!(T))
        .add_trait_generic(quote!(T))
        .to_token_stream()
}
