use super::Implementor;
use crate::node::{Entity, Trait};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

///
/// ENTITY
///

fn entity_get_fields(node: &Entity) -> Vec<Ident> {
    node.sort_keys
        .iter()
        .filter_map(|sk| sk.field.clone())
        .collect::<Vec<_>>()
}

// entity
pub fn entity(node: &Entity, t: Trait) -> TokenStream {
    let mut q = quote!();

    q.extend(composite_key(node));

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}

// composite_key
fn composite_key(node: &Entity) -> TokenStream {
    let fields = entity_get_fields(node);

    // Prepare the quote for setting struct fields based on the provided values slice
    let set_fields = fields.iter().enumerate().map(|(i, ident)| {
        let ident_str = ident.to_string();
        quote! {
            if let Some(value) = values.get(#i) {
                this.#ident = value.parse().map_err(|_| ::mimic::orm::Error::parse_field(#ident_str))?;
            }
        }
    });

    // quote for generating the output vector using the ORM trait to
    // format each field as a primary key
    let format_keys = fields.iter().map(|ident| {
        quote! {
            ::mimic::orm::traits::SortKey::format(&this.#ident)
        }
    });

    // create inner
    let inner = if fields.is_empty() {
        quote!(Ok(Vec::new()))
    } else {
        quote! {
            let mut this = Self::default();
            #(#set_fields)*

            // Collect formatted keys and then take only as many as there are input values
            let keys = vec![#(#format_keys),*];
            let limited_keys = keys.into_iter().take(values.len()).collect::<Vec<_>>();

            Ok(limited_keys)
        }
    };

    quote! {
        fn composite_key(values: &[String]) -> Result<Vec<::std::string::String>, ::mimic::orm::Error> {
            #inner
        }
    }
}

///
/// EntityDyn
///

// entity_dyn
pub fn entity_dyn(node: &Entity, t: Trait) -> TokenStream {
    let mut q = quote! {};

    q.extend(on_create(node));
    q.extend(composite_key_dyn(node));
    q.extend(serialize_dyn(node));

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}

// composite_key_dyn
fn composite_key_dyn(node: &Entity) -> TokenStream {
    let parts = entity_get_fields(node)
        .into_iter()
        .map(|field| quote!(::mimic::orm::traits::SortKey::format(&self.#field)));

    // quote
    quote! {
        fn composite_key_dyn(&self) -> Vec<::std::string::String> {
            vec![#(#parts),*]
        }
    }
}

// on_create
fn on_create(node: &Entity) -> TokenStream {
    //  let primary_key = &node.primary_key;

    quote! {
        fn on_create(&mut self) {
    //        self.#primary_key = ::mimic::orm::traits::PrimaryKey::on_create(&self.#primary_key);
        }
    }
}

// serialize_dyn
fn serialize_dyn(_: &Entity) -> TokenStream {
    quote! {
        fn serialize_dyn(&self) -> Result<Vec<u8>, ::mimic::orm::Error> {
            ::mimic::orm::serialize(&self)
        }
    }
}
