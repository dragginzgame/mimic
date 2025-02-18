use super::ActorBuilder;
use quote::quote;
use syn::{Path, parse_str};

// extend
pub fn extend(builder: &mut ActorBuilder) {
    query_load(builder);
    query_delete(builder);
    query_save(builder);
}

// query_load
fn query_load(builder: &mut ActorBuilder) {
    let mut q = quote!();
    let mut inner = quote!();

    // build inner
    for (entity_path, _) in builder.get_entities() {
        let generic: Path = parse_str(&entity_path).unwrap();

        inner.extend(quote! {
            #entity_path => {
                ::mimic::query::load::<#generic>()
                    .method(query.method)
                    .execute(&DB)
            }
        });
    }

    // query_load
    q.extend(quote! {
        #[::mimic::ic::query]
        pub fn query_load(
            query: ::mimic::query::LoadQuery,
        ) -> Result<::mimic::query::LoadResponseDyn, ::mimic::Error> {

            let res = match query.path.as_str() {
                #inner
                _ => Err(::mimic::orm::OrmError::EntityNotFound(query.path.clone()))?
            }?;

            Ok(res)
        }
    });

    builder.extend(q);
}

// query_delete
fn query_delete(builder: &mut ActorBuilder) {
    let mut q = quote!();

    // doesn't need to match on the entity path
    q.extend(quote! {
        #[::mimic::ic::update]
        pub fn query_delete(
            query: ::mimic::query::DeleteQuery,
        ) -> Result<::mimic::query::DeleteResponse, ::mimic::Error> {
            let executor = ::mimic::query::DeleteExecutor::new(query);
            let res = executor.execute(&DB)?;

            Ok(res)
        }
    });

    builder.extend(q);
}

// query_save
fn query_save(builder: &mut ActorBuilder) {
    let mut q = quote!();
    let mut inner = quote!();

    // build inner
    for (entity_path, _) in builder.get_entities() {
        let generic: Path = parse_str(&entity_path).unwrap();

        inner.extend(quote! {
            #entity_path => builder.from_bytes::<#generic>(&query.bytes),
        });
    }

    q.extend(quote! {
        #[::mimic::ic::update]
        pub fn query_save(
            query: ::mimic::query::SaveQueryDyn
        ) -> Result<::mimic::query::SaveResponse, ::mimic::Error> {
            let builder = ::mimic::query::save(query.mode);
            let query = match query.path.as_str() {
                #inner
                _ => Err(::mimic::orm::OrmError::EntityNotFound(query.path.clone()))?
            }?;

            let res = query.debug().execute(&DB)?;

            Ok(res)
        }
    });

    builder.extend(q);
}
