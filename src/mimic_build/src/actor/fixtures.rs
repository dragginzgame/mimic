use super::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, parse_str};

// extend
pub fn extend(builder: &mut ActorBuilder) {
    fixtures(builder);
}

// fixtures
pub fn fixtures(builder: &mut ActorBuilder) {
    let fixtures_replace_all = fixtures_replace_all(builder);

    let q = quote! {

        // init_fixtures
        pub fn init_fixtures() -> Result<(), ::mimic::Error> {
            fixtures_replace_all()
        }

        // fixtures_replace_helper
        #[allow(dead_code)]
        fn fixtures_replace_helper(
            fixtures: ::mimic::types::FixtureList,
        ) -> Result<(), ::mimic::Error> {
            for entity in fixtures {
                ::mimic::query::replace_dyn()
                    .from_entity_dyn(entity)
                    .debug()
                    .execute(&DB)?;
            }

            Ok(())
        }

        // fixtures_replace_all
        #fixtures_replace_all
    };

    builder.extend(q);
}

// fixtures_replace_all
// replaces every single fixture with the latest version
#[must_use]
pub fn fixtures_replace_all(builder: &ActorBuilder) -> TokenStream {
    let mut inner = Vec::new();

    // stores
    for (entity_path, _) in builder.get_entities() {
        let entity_ident: Path = parse_str(&entity_path).unwrap();
        inner.push(quote! {
            fixtures_replace_helper(<#entity_ident as ::mimic::orm::traits::EntityFixture>::fixtures())?;
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
        pub fn fixtures_replace_all() -> Result<(), ::mimic::Error> {
            #inner
        }
    }
}
