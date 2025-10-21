use crate::prelude::*;
use mimic_common::case::{Case, Casing};

///
/// EntityKindTrait
///

pub struct EntityKindTrait {}

impl Imp<Entity> for EntityKindTrait {
    fn strategy(node: &Entity) -> Option<TraitStrategy> {
        let store = &node.store;
        let pk_field = &node.primary_key.to_string();

        // instead of string literals, reference the inherent const idents
        let field_refs: Vec<Ident> = node
            .fields
            .iter()
            .map(|f| {
                let constant = f.ident.to_string().to_case(Case::Constant);
                format_ident!("{constant}")
            })
            .collect();

        // indexes
        let indexes = &node
            .indexes
            .iter()
            .map(Index::schema_part)
            .collect::<Vec<_>>();

        // static definitions
        let mut q = quote! {
            type Store = #store;
            type Canister = <Self::Store as ::mimic::core::traits::StoreKind>::Canister;

            const ENTITY_ID: u64 = ::mimic::core::hash::fnv1a_64(Self::PATH.as_bytes());
            const PRIMARY_KEY: &'static str = #pk_field;
            const FIELDS: &'static [&'static str]  = &[ #( Self::#field_refs ),* ];
            const INDEXES: &'static [&'static ::mimic::schema::node::Index]  = &[#(&#indexes),*];
        };

        // impls
        q.extend(key(node));

        let tokens = Implementor::new(&node.def, Trait::EntityKind)
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
                .expect("primary key must be convertible to Key")
        }
    }
}
