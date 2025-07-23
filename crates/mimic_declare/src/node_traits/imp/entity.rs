use crate::{
    node::Entity,
    node_traits::{Imp, Implementor, Trait},
    traits::AsMacro,
};
use mimic_common::utils::case::{Case, Casing};
use mimic_schema::types::Cardinality;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::LitStr;

///
/// EntityKindTrait
///

pub struct EntityKindTrait {}

impl Imp<Entity> for EntityKindTrait {
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let store = &node.store;
        let pk_type = &node.fields.get(&node.primary_key).unwrap().value;
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
        q.extend(primary_key(node));
        q.extend(key(node));
        q.extend(values(node));

        let tokens = Implementor::new(node.ident(), Trait::EntityKind)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

// primary_key
fn primary_key(node: &Entity) -> TokenStream {
    let primary_key = &node.primary_key;

    quote! {
        fn primary_key(&self) -> Self::PrimaryKey {
            self.#primary_key
        }
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

///
/// EntityAccessorTrait
///

pub struct EntityAccessorTrait {}

impl Imp<Entity> for EntityAccessorTrait {
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let ident = node.ident(); // e.g., `Indexable`
        let fields_ident_s = format!("{ident}_FIELDS").to_case(Case::UpperSnake);
        let fields_ident = format_ident!("{fields_ident_s}");

        let mut fn_defs = Vec::new();
        let mut field_accessors = Vec::new();

        for field in &node.fields {
            let field_ident = &field.ident;
            let field_str = field_ident.to_string();
            let field_ty = &field.value;

            // Generate static function names
            let cmp_fn_s = format!("{ident}_cmp_{field_ident}").to_case(Case::Snake);
            let cmp_fn = format_ident!("{cmp_fn_s}");
            let search_fn_s = format!("{ident}_search_{field_ident}").to_case(Case::Snake);
            let search_fn = format_ident!("{search_fn_s}");

            // Define the functions
            fn_defs.push(quote! {
                fn #search_fn(x: &#ident, text: &str) -> bool {
                    <#field_ty as ::mimic::core::traits::FieldSearchable>::contains_text(&x.#field_ident, text)
                }

                fn #cmp_fn(a: &#ident, b: &#ident) -> ::std::cmp::Ordering {
                    <#field_ty as ::mimic::core::traits::FieldSortable>::cmp(&a.#field_ident, &b.#field_ident)
                }
            });

            // Use the functions in the static accessor table
            field_accessors.push(quote! {
                ::mimic::core::traits::FieldAccessor {
                    name: #field_str,
                    search: Some(#search_fn),
                    cmp: Some(#cmp_fn),
                }
            });
        }

        let static_def = quote! {
            #(#fn_defs)*

            static #fields_ident: &[::mimic::core::traits::FieldAccessor<#ident>] = &[
                #(#field_accessors),*
            ];
        };

        let trait_impl = quote! {
            fn fields() -> &'static [::mimic::core::traits::FieldAccessor<Self>] {
                #fields_ident
            }
        };

        let mut tokens = static_def;
        tokens.extend(
            Implementor::new(ident, Trait::EntityAccessor)
                .set_tokens(trait_impl)
                .to_token_stream(),
        );

        Some(tokens)
    }
}
