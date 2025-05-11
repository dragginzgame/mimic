use crate::{
    imp::{Imp, Implementor},
    node::{Cardinality, Entity, FieldList, MacroNode, Record, Trait},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

///
/// FieldFilterTrait
///

pub struct FieldFilterTrait {}

///
/// Entity
///

impl Imp<Entity> for FieldFilterTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let q = fields_filter(&node.fields);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Record
///

impl Imp<Record> for FieldFilterTrait {
    fn tokens(node: &Record, t: Trait) -> Option<TokenStream> {
        let q = fields_filter(&node.fields);

        let tokens = Implementor::new(&node.def, t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

// fields_filter
// check if a node's fields are empty and generate an appropriate logical expression
pub fn fields_filter(node: &FieldList) -> TokenStream {
    let matches: Vec<_> = node
        .fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let name_str = name.to_string();

            match field.value.cardinality() {
                Cardinality::One => quote! {
                    #name_str => {
                        if ::mimic::traits::Filterable::contains_text(&self.#name, text) {
                            return true;
                        }
                    }
                },
                Cardinality::Opt => quote! {
                    #name_str => {
                        if let Some(value) = &self.#name {
                            if ::mimic::traits::Filterable::contains_text(value, text) {
                                return true;
                            }
                        }
                    }
                },
                Cardinality::Many => quote!(),
            }
        })
        .collect();

    // Prepare static DEFAULT_FIELDS only once
    let default_fields = node.fields.iter().map(|field| {
        let name = field.name.to_string();
        quote!(#name)
    });
    let default_fields_len = default_fields.len();
    let default_fields_quoted = quote!([#(#default_fields),*]);

    quote! {
        fn list_fields(&self) -> &'static [&'static str] {
            static FIELDS: [&str; #default_fields_len] = #default_fields_quoted;

            &FIELDS
        }

        fn filter_field(&self, field: &str, text: &str) -> bool {
            match field {
                #(#matches)*
                _ => {},
            }

            false
        }
    }
}

///
/// FieldSortTrait
///

pub struct FieldSortTrait {}

///
/// Entity
///

impl Imp<Entity> for FieldSortTrait {
    fn tokens(node: &Entity, t: Trait) -> Option<TokenStream> {
        let q = fields_sort(&node.fields);

        let tokens = Implementor::new(node.def(), t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

///
/// Record
///

impl Imp<Record> for FieldSortTrait {
    fn tokens(node: &Record, t: Trait) -> Option<TokenStream> {
        let q = fields_sort(&node.fields);

        let tokens = Implementor::new(node.def(), t)
            .set_tokens(q)
            .to_token_stream();

        Some(tokens)
    }
}

// fields_sort
fn fields_sort(node: &FieldList) -> TokenStream {
    let mut inner = quote!();

    for field in &node.fields {
        if field.value.cardinality() == Cardinality::Many {
            continue;
        }

        let field_ident = &field.name;
        let field_str = field_ident.to_string();

        inner.extend(quote! {
            #field_str => {
                if matches!(direction, ::mimic::schema::types::SortDirection::Asc) {
                    funcs.push(Box::new(|a, b| ::mimic::traits::Orderable::cmp(&a.#field_ident, &b.#field_ident)));
                } else {
                    funcs.push(Box::new(|a, b| ::mimic::traits::Orderable::cmp(&b.#field_ident, &a.#field_ident)));
                }
            },
        });
    }

    // quote
    let order = &node.order;
    quote! {
        fn default_order() -> Vec<(String, ::mimic::schema::types::SortDirection)> {
            vec![#(#order),*]
        }

        fn generate_sorter(order: &[(String, ::mimic::schema::types::SortDirection)]) -> Box<dyn Fn(&Self, &Self) -> ::std::cmp::Ordering> {
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
