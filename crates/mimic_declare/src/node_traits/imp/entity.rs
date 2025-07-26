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
        let mut q = quote! {
            type Store = #store;
            type PrimaryKey = #pk_type;
            type Indexes = (#(#index_idents),*);

            const PRIMARY_KEY: &'static str = #pk_field;
        };

        // impls
        q.extend(key(node));

        let tokens = Implementor::new(node.ident(), Trait::EntityKind)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

// key
fn key(node: &Entity) -> TokenStream {
    let primary_key = &node.primary_key;

    quote! {
        fn key(&self) -> Key {
            use ::mimic::core::traits::FieldValue;

            self.#primary_key
                .to_value()
                .as_key()
                .unwrap()
        }
    }
}
