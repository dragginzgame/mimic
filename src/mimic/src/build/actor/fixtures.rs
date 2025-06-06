use crate::build::actor::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, parse_str};

// generate
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    let body = generate_replace_all(builder);

    quote! {
        /// Initializes all registered fixtures by replacing data in storage.
        pub fn mimic_init_fixtures() -> Result<(), ::mimic::Error> {
            mimic_fixtures_replace_all()
        }

        /// Helper to replace all fixture data in storage.
        #[allow(dead_code)]
        fn fixtures_replace_helper(
            fixtures: Vec<Box<dyn EntityKindDyn>>,
        ) -> Result<(), ::mimic::Error> {
            for entity in fixtures {
               query_save!().debug().execute(::mimic::data::query::replace().entity_dyn(entity)).unwrap();
            }

            Ok(())
        }

        /// Replaces fixtures for all registered entities.
        #[allow(clippy::too_many_lines)]
        #[allow(clippy::missing_const_for_fn)]
        pub fn mimic_fixtures_replace_all() -> Result<(), ::mimic::Error> {
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
            fixtures_replace_helper(<#entity_ident as ::mimic::traits::EntityFixture>::fixtures())?;
        });
    }

    // quote
    if inner.is_empty() {
        quote!(Ok(()))
    } else {
        let num_entities = inner.len();
        quote! {
            #(#inner)*
            log!(Log::Info, "added fixtures ({} entities)", #num_entities);

            Ok(())
        }
    }
}
