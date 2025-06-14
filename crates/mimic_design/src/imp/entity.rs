use crate::{
    imp::{Imp, Implementor},
    node::{Entity, MacroNode, Trait},
};
use mimic::schema::types::Cardinality;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

///
/// EntityKindTrait
///

pub struct EntityKindTrait {}

impl Imp<Entity> for EntityKindTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let mut q = quote!();

        q.extend(query_values(node));
        q.extend(build_sort_key(node));
        q.extend(sort_key(node));

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

// query_values
fn query_values(node: &Entity) -> TokenStream {
    let entries = node.fields.iter().filter_map(|field| {
        let field_ident = &field.name;
        let field_name = field.name.to_string();
        let item = &field.value.item;

        match field.value.cardinality() {
            Cardinality::One => Some(quote! {
                (#field_name.to_string(), <#item as ::mimic::traits::FieldQueryable>::to_query_value(&self.#field_ident))
            }),

            Cardinality::Opt => Some(quote! {
                (#field_name.to_string(), self.#field_ident
                    .as_ref()
                    .and_then(<#item as ::mimic::traits::FieldQueryable>::to_query_value))
            }),

            Cardinality::Many => None,
        }
    });

    quote! {
        fn query_values(&self) -> ::std::collections::HashMap<String, Option<String>> {
            [
                #(#entries),*
            ].into_iter().collect()
        }
    }
}

// build_sort_key
fn build_sort_key(node: &Entity) -> TokenStream {
    let assignments = node
        .sort_keys
        .iter()
        .enumerate()
        .filter_map(|(i, sk)| {
            let field_type = &sk.field.value.item;
            let index = syn::Index::from(i);

            Some(quote! {
                {
                    let raw = values.get(#index);
                    let parsed: #field_type = <#field_type as ::mimic::traits::ParseSortKeyPart>::parse_sort_key_part(raw)
                        .ok_or_else(|| ::mimic::Error::InvalidSortKeyValue(#index))?;
                    let formatted = parsed.to_sort_key_part()
                        .ok_or_else(|| ::mimic::Error::UnsortableField(#index))?;

                    formatted
                }
            })
        });

    quote! {
        fn build_sort_key(&self, values: &[String]) -> ::mimic::SortKey {
            let parts = vec![
                #(#assignments),*
            ];

            let labels = self.sk_fields()
                .iter()
                .map(|sk| sk.label.clone())
                .collect::<Vec<_>>();

            let key_parts = labels.into_iter().zip(parts.into_iter()).map(|(k, v)| (k, Some(v))).collect();
            ::mimic::SortKey::new(key_parts)
        }
    }
}

// sort_key
fn sort_key(node: &Entity) -> TokenStream {
    let entries = node.sort_keys.iter().map(|sk| {
        let field_ident = &sk.field;

        match field.value.cardinality() {
            Cardinality::One => Some(quote! {
                <#node as ::mimic::traits::FormatSortKey>::format_sort_key(&self.#field_ident)
            }),

            Cardinality::Opt | Cardinality::Many => None,
        }
    });

    quote! {
        fn sort_key(&self) -> Result<Vec<String>, Error> {
            self.build_sort_key(&[
                #(#entries),*
            ])
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

              match field.value.cardinality() {
                    Cardinality::One => quote! {
                        ( #name_str, |s: &#ident, text|
                            ::mimic::traits::FormatQueryValue::contains_text(&s.#name, text)
                        )
                    },
                    Cardinality::Opt => quote! {
                        ( #name_str, |s: &#ident, text|
                            s.#name.as_ref().map_or(false, |v| ::mimic::traits::FormatQueryValue::contains_text(v, text))
                        )
                    },
                    Cardinality::Many => quote! {
                        ( #name_str, |s: &#ident, text|
                             s.#name.iter().any(|v| ::mimic::traits::FormatQueryValue::contains_text(v, text))
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
            if field.value.cardinality() == Cardinality::Many {
                continue;
            }

            let field_ident = &field.name;
            let field_str = field_ident.to_string();
            let asc_fn = format_ident!("asc_{field_ident}");
            let desc_fn = format_ident!("desc_{field_ident}");

            asc_fns.extend(quote! {
                fn #asc_fn(a: &#node_ident, b: &#node_ident) -> ::std::cmp::Ordering {
                    ::mimic::traits::Orderable::cmp(&a.#field_ident, &b.#field_ident)
                }
            });

            desc_fns.extend(quote! {
                fn #desc_fn(a: &#node_ident, b: &#node_ident) -> ::std::cmp::Ordering {
                    ::mimic::traits::Orderable::cmp(&b.#field_ident, &a.#field_ident)
                }
            });

            match_arms.extend(quote! {
                (#field_str, ::mimic::data::SortDirection::Asc) => comps.push(#asc_fn),
                (#field_str, ::mimic::data::SortDirection::Desc) => comps.push(#desc_fn),
            });
        }

        let q = quote! {
            fn sort(order: &[(String, ::mimic::data::SortDirection)])
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
