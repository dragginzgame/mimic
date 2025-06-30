use crate::{
    imp::{Imp, Implementor},
    node::{Entity, MacroNode},
    schema::{Cardinality, Schemable},
    traits::Trait,
};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::LitStr;

///
/// EntityKindTrait
///

pub struct EntityKindTrait {}

impl Imp<Entity> for EntityKindTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let key_size = &node
            .data_keys
            .iter()
            .filter(|dk| dk.field.is_some())
            .count();
        let store = &node.store;
        let defs = node.indexes.iter().map(Schemable::schema);

        // static definitions
        let mut q = quote! {
            type PrimaryKey = [::mimic::core::value::IndexValue; #key_size];

            const STORE: &'static str = #store::PATH;
            const INDEXES: &'static [::mimic::schema::node::EntityIndex] = &[
                #(#defs),*
            ];
        };

        // impls
        q.extend(primary_key(node));
        q.extend(build_data_key(node));
        q.extend(values(node));

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

// primary_key
fn primary_key(node: &Entity) -> TokenStream {
    let fields: Vec<_> = node.data_keys.iter().filter_map(|dk| {
        dk.field.as_ref().map(|field| {
            let field_ident = field;

            quote! {
                self.#field_ident.to_value().into_index_value().expect("primary key field must be indexable")
            }
        })
    }).collect();

    quote! {
        fn primary_key(&self) -> Self::PrimaryKey {
            use ::mimic::core::traits::FieldValue;

            [
                #(#fields),*
            ]
        }
    }
}

// build_data_key
fn build_data_key(node: &Entity) -> TokenStream {
    let mut value_index: usize = 0;

    let parts = node.data_keys.iter().map(|data_key| {
        let entity = &data_key.entity;

        if data_key.field.is_some() {
            let idx = value_index;
            value_index += 1;

            quote! {
                ::mimic::db::store::DataKeyPart::new(
                    #entity::PATH,
                    Some(values[#idx]),
                )
            }
        } else {
            quote! {
                ::mimic::db::store::DataKeyPart::new(
                    #entity::PATH,
                    None,
                )
            }
        }
    });

    quote! {
        fn build_data_key(values: &[::mimic::core::value::IndexValue]) -> ::mimic::db::store::DataKey {

            // Ensure at least one part if none were provided
            if values.is_empty() {
                return vec![::mimic::db::store::DataKeyPart::new(Self::PATH, None)].into();
            }

            let parts = vec![
                #(#parts),*
            ];

            parts.into()
        }
    }
}

// values
fn values(node: &Entity) -> TokenStream {
    let inserts = node
        .fields
        .iter()
        .filter_map(|field| {
            let field_ident = &field.name;
            let field_lit = LitStr::new(&field_ident.to_string(), Span::call_site());

            match *field.value.cardinality() {
                Cardinality::One => Some(quote! {
                    map.insert(#field_lit, self.#field_ident.to_value());
                }),

                Cardinality::Opt => Some(quote! {
                    map.insert(#field_lit,
                        self.#field_ident
                            .as_ref()
                            .map_or(::mimic::core::value::Value::Null, |v| v.to_value())
                    );
                }),

                Cardinality::Many => None, // Optionally: serialize Vec<T> if needed
            }
        })
        .collect::<Vec<_>>();

    let cap = inserts.len();
    quote! {
        fn values(&self) -> ::mimic::core::value::Values {
            use ::mimic::core::traits::FieldValue;

            let mut map = ::std::collections::HashMap::with_capacity(#cap);
            #(#inserts)*

            ::mimic::core::value::Values(map)
        }
    }
}

///
/// EntitySearchTrait
///

pub struct EntitySearchTrait {}

impl Imp<Entity> for EntitySearchTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let ident = &node.def.ident;

        let field_fns: Vec<_> = node
            .fields
            .iter()
            .map(|field| {
                let name = &field.name;
                let name_str = name.to_string();

              match *field.value.cardinality() {
                    Cardinality::One => quote! {
                        ( #name_str, |s: &#ident, text|
                            ::mimic::core::traits::FieldSearchable::contains_text(&s.#name, text)
                        )
                    },
                    Cardinality::Opt => quote! {
                        ( #name_str, |s: &#ident, text|
                            s.#name.as_ref().map_or(false, |v| ::mimic::core::traits::FieldSearchable::contains_text(v, text))
                        )
                    },
                    Cardinality::Many => quote! {
                        ( #name_str, |s: &#ident, text|
                             s.#name.iter().any(|v| ::mimic::core::traits::FieldSearchable::contains_text(v, text))
                        )
                    },
                }
            })
            .collect();

        let q = quote! {
            fn search_field(&self, field: &str, text: &str) -> bool {
                static SEARCH_FIELDS: &[(&str, fn(&#ident, &str) -> bool)] = &[
                    #(#field_fns),*
                ];

                SEARCH_FIELDS
                    .iter()
                    .find(|(name, _)| *name == field)
                    .map(|(_, f)| f(self, text))
                    .unwrap_or(false)
            }
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// EntitySortTrait
///

pub struct EntitySortTrait {}

impl Imp<Entity> for EntitySortTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let node_ident = &node.def.ident;

        let mut asc_fns = quote!();
        let mut desc_fns = quote!();
        let mut match_arms = quote!();

        for field in &node.fields {
            if *field.value.cardinality() == Cardinality::Many {
                continue;
            }

            let field_ident = &field.name;
            let field_str = field_ident.to_string();
            let asc_fn = format_ident!("asc_{field_ident}");
            let desc_fn = format_ident!("desc_{field_ident}");

            asc_fns.extend(quote! {
                fn #asc_fn(a: &#node_ident, b: &#node_ident) -> ::std::cmp::Ordering {
                    ::mimic::core::traits::FieldSortable::cmp(&a.#field_ident, &b.#field_ident)
                }
            });

            desc_fns.extend(quote! {
                fn #desc_fn(a: &#node_ident, b: &#node_ident) -> ::std::cmp::Ordering {
                    ::mimic::core::traits::FieldSortable::cmp(&b.#field_ident, &a.#field_ident)
                }
            });

            match_arms.extend(quote! {
                (#field_str, ::mimic::db::query::SortDirection::Asc) => comps.push(#asc_fn),
                (#field_str, ::mimic::db::query::SortDirection::Desc) => comps.push(#desc_fn),
            });
        }

        let q = quote! {
            fn sort(order: &[(String, ::mimic::db::query::SortDirection)])
                -> Box<dyn Fn(&#node_ident, &#node_ident) -> ::std::cmp::Ordering>
            {
                #asc_fns
                #desc_fns

                let mut comps: Vec<fn(&#node_ident, &#node_ident) -> ::std::cmp::Ordering> = Vec::new();

                for (field, dir) in order {
                    match (field.as_str(), dir) {
                        #match_arms
                        _ => {}
                    }
                }

                Box::new(move |a, b| {
                    for cmp in &comps {
                        let ord = cmp(a, b);
                        if ord != ::std::cmp::Ordering::Equal {
                            return ord;
                        }
                    }
                    ::std::cmp::Ordering::Equal
                })
            }
        };

        let tokens = Implementor::new(node.def(), t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
