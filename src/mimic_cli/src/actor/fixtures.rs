use super::ActorBuilder;
use orm_schema::{build::schema, node::Fixture};
use proc_macro2::TokenStream;
use syn::{parse_str, Path};
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    fixture_actor(builder);
    fixture_module(builder);

    builder.add_hook("actorgen::init_fixtures");
}

// fixture_actor
pub fn fixture_actor(builder: &mut ActorBuilder) {
    let q = quote! {

        // fixtures_replace_all
        #[::mimic::ic::update]
        async fn fixtures_replace_all() -> Result<(), Error> {
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
        pub fn init_fixtures() -> Result<(), Error> {
            fixtures_replace_all()
        }

        // fixtures_replace_helper
        #[allow(dead_code)]
        fn fixtures_replace_helper(
            fixtures: Vec<Box<dyn ::mimic::orm::traits::EntityDynamic>>,
        ) -> Result<(), Error> {
            DB.with(|db| {
                ::mimic::db::query::replace(db)
            //     .debug()
                    .from_entities_dynamic(fixtures)
                    .map_err(Error::from)?;

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
    let schema = schema();
    for (entity_path, _) in builder.get_entities() {
        for (fixture_path, _) in
            schema.filter_nodes::<Fixture, _>(|node| node.entity == entity_path)
        {
            let fixture_ident: Path = parse_str(fixture_path).unwrap();
            inner.push(quote! {
                fixtures_replace_helper(#fixture_ident::fixtures())?;
            });
        }
    }

    // quote
    let inner = if inner.is_empty() {
        quote!(Ok(()))
    } else {
        let num_entities = inner.len();
        quote! {
            log!(Log::Info, "added fixtures ({} entities)", #num_entities);
            #(#inner)*

            Ok(())
        }
    };

    quote! {
        #[allow(clippy::too_many_lines)]
        pub fn fixtures_replace_all() -> Result<(), Error> {
            #inner
        }
    }
}
