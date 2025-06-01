use crate::build::actor::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, parse_str};

// generate
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    fixtures(builder)
}

// fixtures
fn fixtures(builder: &ActorBuilder) -> TokenStream {
    let fixtures_replace_all = fixtures_replace_all(builder);

    quote! {

        // mimic_init_fixtures
        pub fn mimic_init_fixtures() -> Result<(), ::mimic::Error> {
            mimic_fixtures_replace_all()
        }

        // fixtures_replace_helper
        #[allow(dead_code)]
        fn mimic_fixtures_replace_helper(
            fixtures: ::mimic::types::FixtureList,
        ) -> Result<(), ::mimic::Error> {
            for entity in fixtures {
                ::mimic::query::replace()
                    .entity_dyn(entity)
                  //  .debug()
                    .execute(&DB)?;
            }

            Ok(())
        }

        // fixtures_replace_all
        #fixtures_replace_all
    }
}

// fixtures_replace_all
// replaces every single fixture with the latest version
fn fixtures_replace_all(builder: &ActorBuilder) -> TokenStream {
    let mut inner = Vec::new();

    // stores
    for (entity_path, _) in builder.get_entities() {
        let entity_ident: Path = parse_str(&entity_path).unwrap();
        inner.push(quote! {
            mimic_fixtures_replace_helper(<#entity_ident as ::mimic::traits::EntityFixture>::fixtures())?;
        });
    }

    // quote
    let inner = if inner.is_empty() {
        quote!(Ok(()))
    } else {
        let num_entities = inner.len();
        quote! {
            #(#inner)*
            log!(Log::Info, "added fixtures ({} entities)", #num_entities);

            Ok(())
        }
    };

    quote! {
        #[allow(clippy::too_many_lines)]
        #[allow(clippy::missing_const_for_fn)]
        pub fn mimic_fixtures_replace_all() -> Result<(), ::mimic::Error> {
            #inner
        }
    }
}
