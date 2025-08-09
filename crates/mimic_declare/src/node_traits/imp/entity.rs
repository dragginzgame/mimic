use crate::{
    node::{Entity, Index},
    node_traits::{Imp, Implementor, Trait, TraitStrategy},
    traits::{HasIdent, HasSchemaPart, HasTypePart},
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
        let pk_field = &node.primary_key.to_string();

        // future note: don't remove fields, it will be super handy
        let fields: Vec<LitStr> = node
            .fields
            .iter()
            .map(|f| LitStr::new(&f.ident.to_string(), Span::call_site()))
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

            const PRIMARY_KEY: &'static str = #pk_field;
            const FIELDS: &'static [&'static str]  = &[#(#fields),*];
            const INDEXES: &'static [&'static ::mimic::schema::node::Index]  = &[#(&#indexes),*];
        };

        // impls
        q.extend(key(node));

        let mut tokens = Implementor::new(node.ident(), Trait::EntityKind)
            .set_tokens(q)
            .to_token_stream();

        // after impl
        tokens.extend(index_asserts(node));

        // add indexes so the compiler will catch invalid indexes
        // in the format type _ : (FieldKey, FieldKey) = (..., ...)

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

// Build per-index compile-time checks
pub fn index_asserts(node: &Entity) -> TokenStream {
    let per_index: Vec<TokenStream> = node
        .indexes
        .iter()
        .map(|idx| {
            // Each field -> ::full::path
            let field_exprs: Vec<_> = idx
                .fields
                .iter()
                .map(|ident| {
                    let field = node.fields.get(ident).unwrap().value.type_part();

                    quote!( #field )
                })
                .collect();

            quote! {
                #[allow(unused)]
                const _: () = {

                    // Compile-time bound: every key must implement FieldKey + Copy
                    const fn _require_key<K: ::mimic::core::traits::FieldKey>() {}

                    // Assert each field key satisfies the trait bounds
                    #( _require_key::<#field_exprs>(); )*
                };
            }
        })
        .collect();

    quote! { #( #per_index )* }
}
