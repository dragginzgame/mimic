use crate::{
    imp::Implementor,
    node::{Entity, FieldList, MacroNode, Record, Trait},
};
use orm::types::Cardinality;
use proc_macro2::TokenStream;
use quote::ToTokens;

// entity
pub fn entity(node: &Entity, t: Trait) -> TokenStream {
    let q = field_list(&node.fields);

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
}

// record
pub fn record(node: &Record, t: Trait) -> TokenStream {
    let q = field_list(&node.fields);

    Implementor::new(node.def(), t)
        .set_tokens(q)
        .to_token_stream()
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
        fn fields_contain_text(&self, fields: Option<&[String]>, text: &str) -> bool {
            static DEFAULT_FIELDS: [&str; #default_fields_len] = #default_fields_quoted;

            // Use the provided fields or fall back to DEFAULT_FIELDS
            let field_strings = fields.map_or_else(|| DEFAULT_FIELDS
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(), |fields| fields.to_vec());

            for field in field_strings {
                match field.as_str() {
                    #(#matches)*
                    _ => {},
                }
            }

            false
        }
    }
}
