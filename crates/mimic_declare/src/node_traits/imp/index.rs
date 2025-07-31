use crate::{
    node::Index,
    node_traits::{Imp, Implementor, Trait, TraitStrategy},
    traits::HasIdent,
};
use quote::{ToTokens, quote};
use syn::LitStr;

///
/// IndexKindTrait
///

pub struct IndexKindTrait {}

impl Imp<Index> for IndexKindTrait {
    fn strategy(node: &Index) -> Option<TraitStrategy> {
        let store = &node.store;
        let entity = &node.entity;
        let unique = node.unique;
        let field_lits: Vec<LitStr> = node
            .fields
            .iter()
            .map(|f| LitStr::new(&f.to_string(), f.span()))
            .collect();

        // static definitions
        let q = quote! {
            type Store = #store;
            type Entity = #entity;

            const FIELDS: &'static [&'static str] = &[#(#field_lits),*];
            const UNIQUE: bool = #unique;
        };

        let tokens = Implementor::new(node.ident(), Trait::IndexKind)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
    }
}
