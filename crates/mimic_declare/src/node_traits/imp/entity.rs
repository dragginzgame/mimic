use crate::{
    node::Entity,
    node_traits::{Imp, Implementor, Trait, TraitStrategy},
    traits::{HasIdent, HasTypePart},
};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::LitStr;

///
/// EntityKindTrait
///

pub struct EntityKindTrait {}

impl Imp<Entity> for EntityKindTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let store = &node.store;
        let index_idents = &node.indexes;
        let pk_field = &node.primary_key.to_string();

        let pk_type = &node
            .fields
            .get(&node.primary_key)
            .unwrap()
            .value
            .type_part();

        // don't remove fields, it will be super handy
        let fields: Vec<LitStr> = node
            .fields
            .iter()
            .map(|f| LitStr::new(&f.ident.to_string(), Span::call_site()))
            .collect();

        // static definitions
        let mut q = quote! {
            type Store = #store;
            type PrimaryKey = #pk_type;
            type Indexes = (#(#index_idents),*);

            const PRIMARY_KEY: &'static str = #pk_field;
            const FIELDS: &'static [&'static str]  = &[#(#fields),*];
        };

        // impls
        q.extend(key(node));

        let tokens = Implementor::new(node.ident(), Trait::EntityKind)
            .set_tokens(q)
            .to_token_stream();

        Some(TraitStrategy::from_impl(tokens))
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
