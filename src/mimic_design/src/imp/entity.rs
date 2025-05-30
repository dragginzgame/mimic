use crate::{
    imp::{Imp, Implementor},
    node::{Entity, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

///
/// EntityTrait
///

pub struct EntityTrait {}

///
/// Entity
///

impl Imp<Entity> for EntityTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let store = &node.store;
        let q = quote! {
            const STORE: &'static str = <#store as ::mimic::traits::Path>::PATH;
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// EntityDynTrait
///

pub struct EntityDynTrait {}

///
/// Entity
///

impl Imp<Entity> for EntityDynTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let mut q = quote!();

        q.extend(id(node));
        q.extend(composite_key(node));
        q.extend(store(node));

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

// id
fn id(node: &Entity) -> TokenStream {
    let last_sk = node.sort_keys.last().expect("no sort keys!");
    let inner = if let Some(field) = last_sk.field.as_ref() {
        quote! {
            Some(::mimic::traits::SortKeyValue::format(&self.#field))
        }
    } else {
        quote!(None)
    };

    quote! {
        fn id(&self) -> Option<String> {
            #inner
        }
    }
}

// composite_key
fn composite_key(node: &Entity) -> TokenStream {
    let parts = entity_get_fields(node)
        .into_iter()
        .map(|field| quote!(::mimic::traits::SortKeyValue::format(&self.#field)));

    // quote
    quote! {
        fn composite_key(&self) -> Vec<::std::string::String> {
            vec![#(#parts),*]
        }
    }
}

// store
fn store(node: &Entity) -> TokenStream {
    let store = &node.store;

    quote! {
        fn store(&self) -> String {
            <#store as ::mimic::traits::Path>::PATH.to_string()
        }
    }
}

///
/// Helper
///

fn entity_get_fields(node: &Entity) -> Vec<Ident> {
    node.sort_keys
        .iter()
        .filter_map(|sk| sk.field.clone())
        .collect::<Vec<_>>()
}
