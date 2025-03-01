use super::Implementor;
use crate::node::{Cardinality, Entity, FieldList, Record, Trait};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

// entity
pub fn entity(node: &Entity, t: Trait) -> Option<TokenStream> {
    let q = field_list(&node.fields);

    let tokens = Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}

// record
pub fn record(node: &Record, t: Trait) -> Option<TokenStream> {
    let q = field_list(&node.fields);

    let tokens = Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream();

    Some(tokens)
}

// field_list
// check if a node's fields are empty and generate an appropriate logical expression
pub fn field_list(node: &FieldList) -> TokenStream {
    let matches: Vec<_> = node
        .fields
        .iter()
        .map(|field| {
            let name = &field.name;
            let name_str = name.to_string();

            match field.value.cardinality() {
                Cardinality::One => quote! {
                    #name_str => {
                        if ::mimic::orm::traits::Filterable::contains_text(&self.#name, text) {
                            return true;
                        }
                    }
                },
                Cardinality::Opt => quote! {
                    #name_str => {
                        if let Some(value) = &self.#name {
                            if ::mimic::orm::traits::Filterable::contains_text(value, text) {
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
