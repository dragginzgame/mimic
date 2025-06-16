use crate::{
    imp::{Imp, Implementor},
    node::{Entity, MacroNode, Trait},
    traits::Schemable,
};
use mimic::schema::types::Cardinality;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};

///
/// EntityKindTrait
///

pub struct EntityKindTrait {}

impl Imp<Entity> for EntityKindTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        // quote
        let store = &node.store;
        let mut q = quote! {
            const STORE: &'static str = #store::PATH;
        };

        q.extend(indexes(node));
        q.extend(key(node));
        q.extend(values(node));
        q.extend(build_sort_key(node));
        //    q.extend(build_index_key(node));

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

// indexes
fn indexes(node: &Entity) -> TokenStream {
    let defs = node.indexes.iter().map(|index| index.schema());

    quote! {
        const INDEXES: &'static [::mimic::schema::node::EntityIndex] = &[
            #(#defs),*
        ];
    }
}

// key
fn key(node: &Entity) -> TokenStream {
    let fields = node.sort_keys.iter().filter_map(|sk| {
        sk.field
            .as_ref()
            .map(|field| quote!(self.#field.to_string()))
    });

    quote! {
        fn key(&self) -> ::mimic::types::Key {
            Key(vec![
                #(#fields),*
            ])
        }
    }
}

// values
fn values(node: &Entity) -> TokenStream {
    let inserts = node.fields.iter().filter_map(|field| {
        let field_ident = &field.name;
        let field_lit = syn::LitStr::new(&field_ident.to_string(), Span::call_site());

        match field.value.cardinality() {
            Cardinality::One => Some(quote! {
                if let Some(v) = self.#field_ident.to_query_value() {
                    map.insert(#field_lit, Some(v));
                }
            }),

            Cardinality::Opt => Some(quote! {
                if let Some(inner) = &self.#field_ident {
                    if let Some(v) = inner.to_query_value() {
                        map.insert(#field_lit, Some(v));
                    }
                } else {
                    map.insert(#field_lit, None);
                }
            }),

            Cardinality::Many => None,
        }
    });

    quote! {
        fn values(&self) -> ::mimic::def::EntityValues {
            use ::mimic::def::traits::FieldQueryable;

            let mut map = ::std::collections::HashMap::with_capacity(3);
            #(#inserts)*

            map.into()
        }
    }
}

// build_sort_key
fn build_sort_key(node: &Entity) -> TokenStream {
    // parts
    let parts = node.sort_keys.iter().map(|sort_key| {
        let entity = &sort_key.entity;

        match &sort_key.field {
            Some(field) => quote! {
                ::mimic::db::types::SortKeyPart::new(
                    #entity::PATH,
                    this.#field.to_sort_key_part(),
                )
            },
            None => quote! {
                ::mimic::db::types::SortKeyPart::new(
                    #entity::PATH,
                    None,
                )
            },
        }
    });

    // set_fields
    let mut index: usize = 0;
    let set_fields = node.sort_keys.iter().filter_map(|sort_key| {
        let field = sort_key.field.as_ref()?;
        let i = index;
        index += 1;

        Some(quote! {
            if let Some(value) = values.get(#i) {
                this.#field = value.parse().unwrap_or_default();
            }
        })
    });

    // inner
    let inner = quote! {
        use ::mimic::def::traits::FieldSortKey;

        // Ensure at least one part if none were provided
        if values.is_empty() {
            return vec![::mimic::db::types::SortKeyPart::new(Self::PATH, None)].into();
        }

        let mut this = Self::default();
        #(#set_fields)*

        vec![#(#parts),*].into()
    };

    quote! {
        fn build_sort_key(values: &[::std::string::String]) -> ::mimic::db::types::SortKey {
            #inner
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
                            ::mimic::def::traits::FieldQueryable::contains_text(&s.#name, text)
                        )
                    },
                    Cardinality::Opt => quote! {
                        ( #name_str, |s: &#ident, text|
                            s.#name.as_ref().map_or(false, |v| ::mimic::def::traits::FieldQueryable::contains_text(v, text))
                        )
                    },
                    Cardinality::Many => quote! {
                        ( #name_str, |s: &#ident, text|
                             s.#name.iter().any(|v| ::mimic::def::traits::FieldQueryable::contains_text(v, text))
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
                    ::mimic::def::traits::FieldOrderable::cmp(&a.#field_ident, &b.#field_ident)
                }
            });

            desc_fns.extend(quote! {
                fn #desc_fn(a: &#node_ident, b: &#node_ident) -> ::std::cmp::Ordering {
                    ::mimic::def::traits::FieldOrderable::cmp(&b.#field_ident, &a.#field_ident)
                }
            });

            match_arms.extend(quote! {
                (#field_str, ::mimic::db::types::SortDirection::Asc) => comps.push(#asc_fn),
                (#field_str, ::mimic::db::types::SortDirection::Desc) => comps.push(#desc_fn),
            });
        }

        let q = quote! {
            fn sort(order: &[(String, ::mimic::db::types::SortDirection)])
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
