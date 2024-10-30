use super::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    fixture_actor(builder);
    fixture_module(builder);

    builder.add_init_hook("actorgen::init_fixtures");
}

// fixture_actor
pub fn fixture_actor(builder: &mut ActorBuilder) {
    let q = quote! {

        // fixtures_replace_all
        #[::mimic::ic::update]
        #[allow(clippy::missing_const_for_fn)]
        async fn fixtures_replace_all() -> Result<(), ::mimic::api::Error> {
            guard(vec![Guard::Controller]).await?;

            actorgen::fixtures_replace_all()?;

            Ok(())
        }
    };

    builder.extend_actor(q);
}

// fixture_module
pub fn fixture_module(builder: &mut ActorBuilder) {
    let fixtures_replace_all = fixtures_replace_all(builder);

    let q = quote! {

        // init_fixtures
        pub fn init_fixtures() -> Result<(), ::mimic::api::Error> {
            fixtures_replace_all()
        }

        // fixtures_replace_helper
        #[allow(dead_code)]
        fn fixtures_replace_helper(
            fixtures: Vec<Box<dyn ::mimic::orm::traits::EntityDynamic>>,
        ) -> Result<(), ::mimic::api::Error> {
            DB.with(|db| {
                ::mimic::db::query::replace(db)
            //     .debug()
                    .from_entities_dynamic(fixtures)?;

                Ok(())
            })
        }

        // fixtures_replace_all
        #fixtures_replace_all
    };

    builder.extend_module(q);
}

// fixtures_replace_all
// replaces every single fixture with the latest version
#[must_use]
pub fn fixtures_replace_all(builder: &ActorBuilder) -> TokenStream {
    let mut inner = Vec::new();

    // stores
    for (entity_path, _) in builder.get_entities() {
        inner.push(quote! {
            fixtures_replace_helper(#entity_path::fixtures())?;
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
        pub fn fixtures_replace_all() -> Result<(), ::mimic::api::Error> {
            #inner
        }
    }
}
