use crate::{
    imp::{Imp, Implementor},
    node::{Entity, MacroNode, Trait},
};
use mimic_common::types::Cardinality;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

///
/// EntityTrait
///

pub struct EntityTrait {}

impl Imp<Entity> for EntityTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let store = &node.store;
        let q = quote! {
            const STORE: &'static str = <#store as ::mimic::traits::Path>::PATH;
        };

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// EntityDynTrait
///

pub struct EntityDynTrait {}

impl Imp<Entity> for EntityDynTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let mut q = quote!();

        q.extend(id(node));
        q.extend(composite_key(node));
        q.extend(store(node));

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

// id
fn id(node: &Entity) -> TokenStream {
    let last_sk = node.sort_keys.last().expect("no sort keys!");
    let inner = if let Some(field) = last_sk.field.as_ref() {
        quote! {
            Some(::mimic::traits::SortKeyValue::format(&self.#field))
        }
    } else {
        quote!(None)
    };

    quote! {
        fn id(&self) -> Option<String> {
            #inner
        }
    }
}

// composite_key
fn composite_key(node: &Entity) -> TokenStream {
    let parts = node
        .sort_keys
        .iter()
        .filter_map(|sk| sk.field.clone())
        .map(|field| quote!(::mimic::traits::SortKeyValue::format(&self.#field)));

    // quote
    quote! {
        fn composite_key(&self) -> Vec<::std::string::String> {
            vec![#(#parts),*]
        }
    }
}

// store
fn store(node: &Entity) -> TokenStream {
    let store = &node.store;

    quote! {
        fn store(&self) -> String {
            <#store as ::mimic::traits::Path>::PATH.to_string()
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
                            ::mimic::traits::Searchable::contains_text(&s.#name, text)
                        )
                    },
                    Cardinality::Opt => quote! {
                        ( #name_str, |s: &#ident, text|
                            s.#name.as_ref().map_or(false, |v| ::mimic::traits::Searchable::contains_text(v, text))
                        )
                    },
                    Cardinality::Many => quote! {
                        ( #name_str, |s: &#ident, text|
                             s.#name.iter().any(|v| ::mimic::traits::Searchable::contains_text(v, text))
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
                (#field_str, ::mimic::common::types::SortDirection::Asc) => comps.push(#asc_fn),
                (#field_str, ::mimic::common::types::SortDirection::Desc) => comps.push(#desc_fn),
            });
        }

        let q = quote! {
            fn sort(order: &[(String, ::mimic::common::types::SortDirection)])
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
