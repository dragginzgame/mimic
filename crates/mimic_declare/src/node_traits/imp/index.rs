use crate::{
    node::Index,
    node_traits::{Imp, Implementor, Trait},
    traits::AsMacro,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::LitStr;

///
/// IndexKindTrait
///

pub struct IndexKindTrait {}

impl Imp<Index> for IndexKindTrait {
    fn tokens(node: &Index) -> Option<TokenStream> {
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

        Some(tokens)
    }
}
