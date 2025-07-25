use crate::{
    node::Entity,
    node_traits::{Imp, Implementor, Trait},
    traits::{HasIdent, HasTypePart},
};
use mimic_schema::types::Cardinality;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::LitStr;

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
        let pk_field = &node.primary_key.to_string();
        let index_idents = &node.indexes;

        // static definitions
        let mut q = quote! {
            type Store = #store;
            type PrimaryKey = #pk_type;
            type Indexes = (#(#index_idents),*);

            const PRIMARY_KEY: &'static str = #pk_field;
        };

        // impls
        q.extend(key(node));
        q.extend(values(node));

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
                .expect("primary key field must be indexable")
        }
    }
}

// values
fn values(node: &Entity) -> TokenStream {
    let inserts = &node
        .fields
        .iter()
        .map(|field| {
            let field_ident = &field.ident;
            let field_lit = LitStr::new(&field_ident.to_string(), Span::call_site());

            match field.value.cardinality() {
                Cardinality::One => Some(quote! {
                    map.insert(#field_lit, self.#field_ident.to_value());
                }),

                Cardinality::Opt => Some(quote! {
                    map.insert(#field_lit,
                        self.#field_ident
                            .as_ref()
                            .map(|v| v.to_value())
                            .unwrap_or(::mimic::core::value::Value::None)
                    );
                }),

                Cardinality::Many => Some(quote! {
                    let list = self.#field_ident
                        .iter()
                        .map(|v| Box::new(v.to_value()))
                        .collect::<Vec<_>>();

                    map.insert(#field_lit, ::mimic::core::value::Value::List(list));
                }),
            }
        })
        .collect::<Vec<_>>();

    let cap = inserts.len();
    quote! {
        fn values(&self) -> ::mimic::core::value::ValueMap {
            use ::mimic::core::traits::FieldValue;

            let mut map = ::std::collections::HashMap::with_capacity(#cap);
            #(#inserts)*

            ::mimic::core::ValueMap(map)
        }
    }
}
