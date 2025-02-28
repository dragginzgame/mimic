use super::Implementor;
use crate::node::{List, MacroNode, Map, Newtype, PrimitiveGroup, PrimitiveType, Trait, Tuple};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// List
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
/// Newtype
///

//
// newtype
//
// possibility to optimise here as we do have fine-grained control over the
// From implementation for each PrimitiveType
//
pub fn newtype(node: &Newtype, t: Trait) -> TokenStream {
    let mut q = quote!();

    match node.primitive.map(PrimitiveType::group) {
        Some(PrimitiveGroup::Blob | PrimitiveGroup::Integer | PrimitiveGroup::Decimal) => {
            q.extend(newtype_into_inner(node, t));
        }
        Some(PrimitiveGroup::String) => {
            q.extend(newtype_inner(node, t));
            q.extend(newtype_str(node, t));
        }
        // catch-all
        Some(PrimitiveGroup::Bool | PrimitiveGroup::Float | PrimitiveGroup::Ulid) | None => {
            q.extend(newtype_inner(node, t));
        }
        Some(PrimitiveGroup::Unit) => {}
    }

    q
}

// newtype_inner
pub fn newtype_inner(node: &Newtype, t: Trait) -> TokenStream {
    let mut imp = Implementor::new(node.def(), t);

    // match cardinality
    let item = &node.item;
    imp = imp.add_trait_generic(quote!(#item));
    let tokens = quote! {
        fn from(item: #item) -> Self {
            Self(item)
        }
    };

    imp.set_tokens(tokens).to_token_stream()
}

// newtype_into_inner
pub fn newtype_into_inner(node: &Newtype, t: Trait) -> TokenStream {
    let item = &node.item;

    let mut imp = Implementor::new(node.def(), t);
    imp = imp.add_trait_generic(quote!(T));

    let tokens = quote! {
        fn from(t: T) -> Self {
            Self(t.into())
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
    imp = imp.add_trait_generic(quote!(&str));

    // tokens
    let tokens = quote! {
        fn from(s: &str) -> Self {
            Self(s.into())
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
