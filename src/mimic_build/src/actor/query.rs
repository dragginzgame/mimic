use super::ActorBuilder;
use quote::quote;
use syn::{Path, parse_str};

// extend
pub fn extend(builder: &mut ActorBuilder) {
    query_load(builder);
    query_delete(builder);
    //   query_save(builder);
}

//
// query_load
//

fn query_load(builder: &mut ActorBuilder) {
    let entities = builder.get_entities();

    let inner = if entities.is_empty() {
        // If there are no entities, insert a dummy match arm
        quote! {
            Err(::mimic::orm::OrmError::EntityNotFound(path.clone()).into())
        }
    } else {
        // Otherwise, generate match arms dynamically
        let mut load_entities = quote!();
        for (entity_path, _) in entities {
            let generic: Path = parse_str(&entity_path).unwrap();
            load_entities.extend(quote! {
                #entity_path => {
                    executor.execute::<#generic>(&DB)?.as_dynamic()
                }
            });
        }

        quote! {
            let executor = ::mimic::query::LoadExecutor::new(query.clone());
            let path = &query.path;
            let res = match path.as_str() {
                #load_entities
                _ => return Err(::mimic::orm::OrmError::EntityNotFound(path.to_string()).into())
            }?;

            Ok(res)
        }
    };

    // Generate the function
    let q = quote! {
        #[::mimic::ic::query]
        pub fn query_load(
            query: ::mimic::query::LoadQuery,
        ) -> Result<::mimic::query::LoadResponseDyn, ::mimic::Error> {
            #inner
        }
    };

    builder.extend(q);
}

//
// query_delete
//

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

//
// query_save
//
/*
fn query_save(builder: &mut ActorBuilder) {
    let mut q = quote!();
    let mut inner = quote!();

    // build inner
    for (entity_path, _) in builder.get_entities() {
        let generic: Path = parse_str(&entity_path).unwrap();

        inner.extend(quote! {
            #entity_path => builder.from_bytes::<#generic>(&query.bytes)
                .map_err(::mimic::query::QueryError::SaveError)
        });
    }

    // function
    q.extend(quote! {
        #[::mimic::ic::update]
        pub fn query_save(
            query: ::mimic::query::SaveQueryDyn
        ) -> Result<::mimic::query::SaveResponse, ::mimic::Error> {
            let executor = ::mimic::query::SaveExecutorDyn(query);

            let query = match query.path.as_str() {
                #inner
                _ => Err(::mimic::orm::OrmError::EntityNotFound(query.path.clone()))
            }?;

            let res = executor.execute(&DB)?;

            Ok(res)
        }
    });

    builder.extend(q);
}
*/
