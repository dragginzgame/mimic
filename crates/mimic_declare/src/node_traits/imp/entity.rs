use crate::{
    node::Entity,
    node_traits::{Imp, Implementor, Trait},
    traits::{AsMacro, AsSchema},
};
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
        let defs = node.indexes.iter().map(AsSchema::schema);

        // static definitions
        let mut q = quote! {
            type PrimaryKey = #pk_type;

            const STORE: &'static str = #store::PATH;
            const PRIMARY_KEY: &'static str = #pk_field;
            const INDEXES: &'static [::mimic::schema::node::EntityIndex] = &[
                #(#defs),*
            ];
        };

        // impls
        q.extend(primary_key(node));
        q.extend(key(node));
        q.extend(values(node));

        let tokens = Implementor::new(&node.def, Trait::EntityKind)
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
            let field_ident = &field.name;
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
/// EntitySearchTrait
///

pub struct EntitySearchTrait {}

impl Imp<Entity> for EntitySearchTrait {
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let ident = &node.def.ident;

        let field_fns: Vec<_> = node
            .fields
            .iter()
            .map(|field| {
                let name = &field.name;
                let name_str = name.to_string();

                quote! {
                    ( #name_str, |s: &#ident, text| {
                        ::mimic::core::traits::FieldSearchable::contains_text(&s.#name, text)
                    })
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

        let tokens = Implementor::new(&node.def, Trait::EntitySearch)
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
    fn tokens(node: &Entity) -> Option<TokenStream> {
        let node_ident = &node.def.ident;

        let mut asc_fns = quote!();
        let mut desc_fns = quote!();
        let mut match_arms = quote!();

        for field in &node.fields {
            if field.value.cardinality() == Cardinality::Many {
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
            fn sort(expr: &::mimic::db::query::SortExpr)
                -> Box<dyn Fn(&#node_ident, &#node_ident) -> ::std::cmp::Ordering>
            {
                #asc_fns
                #desc_fns

                let mut comps: Vec<fn(&#node_ident, &#node_ident) -> ::std::cmp::Ordering> = Vec::new();

                for (field, dir) in expr.iter() {
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

        let tokens = Implementor::new(node.def(), Trait::EntitySort)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}
