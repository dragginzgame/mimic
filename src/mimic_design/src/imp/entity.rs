use crate::{
    imp::{Imp, Implementor},
    node::{Entity, MacroNode, Trait},
};
use mimic::schema::types::Cardinality;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

///
/// EntityKindDynTrait
///

pub struct EntityKindDynTrait {}

impl Imp<Entity> for EntityKindDynTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let mut q = quote!();

        q.extend(values_string(node));

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

fn values_string(node: &Entity) -> TokenStream {
    quote!()
    /*
    let parts = node.fields.iter().filter_map(|field| {
        if field.value.is_stringable() {
            let ident = &field.ident;
            let name = field.name.to_string();

            Some(quote! {
                map.insert(#name.to_string(), ::mimic::traits::StringValue::to_string_value(&self.#ident));
            })
        } else {
            None
        }
    });

    quote! {
        fn values_string(&self) -> ::std::collections::HashMap<String, String> {
            let mut map = ::std::collections::HashMap::new();
            #(#parts)*

            map
        }
    }*/
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
                (#field_str, ::mimic::schema::types::SortDirection::Asc) => comps.push(#asc_fn),
                (#field_str, ::mimic::schema::types::SortDirection::Desc) => comps.push(#desc_fn),
            });
        }

        let q = quote! {
            fn sort(order: &[(String, ::mimic::schema::types::SortDirection)])
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
