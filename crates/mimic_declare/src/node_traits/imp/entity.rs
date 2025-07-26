use crate::{
    node::Entity,
    node_traits::{Imp, Implementor, Trait},
    traits::{HasIdent, HasTypePart},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// EntityKindTrait
///

pub struct EntityKindTrait {}

impl Imp<Entity> for EntityKindTrait {
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let store = &node.store;
        let pk_type = &node
            .fields
            .get(&node.primary_key)
            .unwrap()
            .value
            .type_part();

        let index_idents = &node.indexes;
        let pk_field = &node.primary_key.to_string();

        // static definitions
        let q = quote! {
            type Store = #store;
            type PrimaryKey = #pk_type;
            type Indexes = (#(#index_idents),*);

            const PRIMARY_KEY: &'static str = #pk_field;
        };

        let tokens = Implementor::new(node.ident(), Trait::EntityKind)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
