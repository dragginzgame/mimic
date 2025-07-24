use crate::{
    node::Canister,
    node_traits::{Imp, Implementor, Trait},
    traits::HasIdent,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// CanisterKindTrait
///

pub struct CanisterKindTrait {}

impl Imp<Canister> for CanisterKindTrait {
    fn tokens(node: &Canister) -> Option<TokenStream> {
        // static definitions
        let q = quote! {};

        let tokens = Implementor::new(node.ident(), Trait::CanisterKind)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
