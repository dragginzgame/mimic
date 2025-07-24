use crate::{
    node::Store,
    node_traits::{Imp, Implementor, Trait},
    traits::HasIdent,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// StoreKindTrait
///

pub struct StoreKindTrait {}

impl Imp<Store> for StoreKindTrait {
    fn tokens(node: &Store) -> Option<TokenStream> {
        let canister = &node.canister;

        // static definitions
        let q = quote! {
            type Canister = #canister;
        };

        let tokens = Implementor::new(node.ident(), Trait::StoreKind)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
