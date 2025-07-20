use crate::{
    node::Entity,
    node_traits::{Imp, Implementor, Trait},
    traits::AsMacro,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// HasIndexesTrait
///

pub struct HasIndexesTrait {}

///
/// Entity
///

impl Imp<Entity> for HasIndexesTrait {
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let index_idents = &node.indexes;

        let q = quote! {
            type Indexes = (#(#index_idents),*);
        };

        let tokens = Implementor::new(node.ident(), Trait::HasIndexes)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
