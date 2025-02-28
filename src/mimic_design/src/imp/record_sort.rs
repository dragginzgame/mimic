use super::Implementor;
use crate::node::{Cardinality, Entity, FieldList, MacroNode, Record, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

// entity
pub fn entity(node: &Entity, t: Trait) -> Option<TokenStream> {
    let q = field_list(&node.fields);

    let tokens = Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}

// record
pub fn record(node: &Record, t: Trait) -> Option<TokenStream> {
    let q = field_list(&node.fields);

    let tokens = Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}

// field_list
fn field_list(node: &FieldList) -> TokenStream {
    let mut inner = quote!();

    for field in &node.fields {
        if field.value.cardinality() == Cardinality::Many {
            continue;
        }

        let field_ident = &field.name;
        let field_str = field_ident.to_string();

        inner.extend(quote! {
            #field_str => {
                if matches!(direction, ::mimic::types::SortDirection::Asc) {
                    funcs.push(Box::new(|a, b| ::mimic::orm::traits::Orderable::cmp(&a.#field_ident, &b.#field_ident)));
                } else {
                    funcs.push(Box::new(|a, b| ::mimic::orm::traits::Orderable::cmp(&b.#field_ident, &a.#field_ident)));
                }
            },
        });
    }

    // quote
    let order = &node.order;
    quote! {
        fn default_order() -> Vec<(String, ::mimic::types::SortDirection)> {
            vec![#(#order),*]
        }

        fn generate_sorter(order: &[(String, ::mimic::types::SortDirection)]) -> Box<dyn Fn(&Self, &Self) -> ::std::cmp::Ordering> {
            let mut funcs: Vec<Box<dyn Fn(&Self, &Self) -> ::std::cmp::Ordering>> = Vec::new();

            for (field, direction) in order {
                match field.as_str() {
                    #inner
                    _ => (),
                }
            }

            Box::new(move |a, b| {
                for func in &funcs {
                    let result = func(a, b);

                    if result != ::std::cmp::Ordering::Equal {
                        return result;
                    }
                }

                ::std::cmp::Ordering::Equal
            })
        }
    }
}
