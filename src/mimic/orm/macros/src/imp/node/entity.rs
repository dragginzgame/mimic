use crate::{
    imp::Implementor,
    node::{Entity, Trait},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

///
/// Entity
///

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
    let fields = node
        .sort_keys
        .iter()
        .flat_map(|sk| sk.fields.iter())
        .chain(&node.primary_keys)
        .collect::<Vec<_>>();

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
            ::mimic::orm::traits::PrimaryKey::format(&this.#ident)
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
/// EntityDynamic
///

// entity_dynamic
pub fn entity_dynamic(node: &Entity, t: Trait) -> TokenStream {
    let mut q = quote! {};

    q.extend(on_create(node));
    q.extend(composite_key_dyn(node));
    q.extend(path_dyn(node));
    q.extend(serialize_dyn(node));

    Implementor::new(&node.def, t)
        .set_tokens(q)
        .to_token_stream()
}

// composite_key_dyn
fn composite_key_dyn(node: &Entity) -> TokenStream {
    let parts = node
        .sort_keys
        .iter()
        .flat_map(|sk| sk.fields.iter())
        .chain(&node.primary_keys)
        .map(|field| quote!(::mimic::orm::traits::PrimaryKey::format(&self.#field)));

    // quote
    quote! {
        fn composite_key_dyn(&self) -> Vec<::std::string::String> {
            vec![#(#parts),*]
        }
    }
}

// on_create
fn on_create(node: &Entity) -> TokenStream {
    let mut inner = quote!();
    for pk in &node.primary_keys {
        inner.extend(quote! {
            self.#pk = ::mimic::orm::traits::PrimaryKey::on_create(&self.#pk);
        });
    }

    quote! {
        fn on_create(&mut self) {
            #inner
        }
    }
}

// path_dyn
fn path_dyn(_: &Entity) -> TokenStream {
    quote! {
        fn path_dyn(&self) -> String {
            <Self as ::mimic::orm::traits::Path>::path()
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
