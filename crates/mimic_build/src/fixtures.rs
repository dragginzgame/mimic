use crate::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, parse_str};

// generate
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    let body = generate_replace_all(builder);

    quote! {
        /// Initializes all registered fixtures by replacing data in storage.
        #[allow(clippy::too_many_lines)]
        pub fn mimic_init_fixtures() -> Result<(), ::mimic::Error> {
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
        let entity_ident: Path = parse_str(&entity_path)
            .unwrap_or_else(|_| panic!("invalid entity path: {entity_path}"));
        inner.push(quote! {
            #entity_ident::insert_fixtures(db);
        });
    }

    // quote
    if inner.is_empty() {
        quote!(Ok(()))
    } else {
        let num_entities = inner.len();

        quote! {
            let db = db!();

            #(#inner)*
            ::icu::log!(::icu::Log::Info, "📦 added fixtures ({} entities)", #num_entities);

            Ok(())
        }
    }
}
