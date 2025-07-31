use crate::{
    node::Store,
    node_traits::{Imp, Implementor, Trait, TraitStrategy},
    traits::HasIdent,
};
use quote::{ToTokens, quote};

///
/// StoreKindTrait
///

pub struct StoreKindTrait {}

impl Imp<Store> for StoreKindTrait {
    fn strategy(node: &Store) -> Option<TraitStrategy> {
        let canister = &node.canister;

        // static definitions
        let q = quote! {
            type Canister = #canister;
        };

        let tokens = Implementor::new(node.ident(), Trait::StoreKind)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
