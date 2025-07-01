use crate::actor::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, parse_str};

// generate
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    let body = generate_replace_all(builder);

    quote! {
        /// Initializes all registered fixtures by replacing data in storage.
        pub fn mimic_init_fixtures() -> Result<(), ::mimic::MimicError> {
            mimic_fixtures_replace_all()
        }

        /// Replaces fixtures for all registered entities.
        #[allow(clippy::too_many_lines)]
        pub fn mimic_fixtures_replace_all() -> Result<(), ::mimic::MimicError> {
            #body
        }
    }
}

// generate_replace_all
// replaces every single fixture with the latest version
fn generate_replace_all(builder: &ActorBuilder) -> TokenStream {
    let mut inner = Vec::new();

    // stores
    for (entity_path, _) in builder.get_entities() {
        let entity_ident: Path = parse_str(&entity_path).unwrap();
        inner.push(quote! {
            #entity_ident::insert_fixtures(&mut exec);
        });
    }

    // quote
    if inner.is_empty() {
        quote!(Ok(()))
    } else {
        let num_entities = inner.len();

        quote! {
            let mut exec = db!().save();

            #(#inner)*
            log!(Log::Info, "added fixtures ({} entities)", #num_entities);

            Ok(())
        }
    }
}
