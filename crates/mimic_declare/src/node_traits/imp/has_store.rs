use crate::{
    node::{Entity, Index},
    node_traits::{Imp, Implementor, Trait},
    traits::AsMacro,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// HasStoreTrait
///

pub struct HasStoreTrait {}

///
/// Entity
///

impl Imp<Entity> for HasStoreTrait {
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let store = &node.store;

        let q = quote! {
            type Store = #store;
        };

        let tokens = Implementor::new(node.ident(), Trait::HasStore)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Index
///

impl Imp<Index> for HasStoreTrait {
    fn tokens(node: &Index) -> Option<TokenStream> {
        let store = &node.store;

        let q = quote! {
            type Store = #store;
        };

        let tokens = Implementor::new(node.ident(), Trait::HasStore)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
