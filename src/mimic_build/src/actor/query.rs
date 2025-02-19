use super::ActorBuilder;
use quote::quote;
use syn::{Path, parse_str};

// extend
pub fn extend(builder: &mut ActorBuilder) {
    query_load(builder);
    query_delete(builder);
    query_save(builder);
}

//
// query_load
//

fn query_load(builder: &mut ActorBuilder) {
    let entities = builder.get_entities();

    let inner = if entities.is_empty() {
        // if there are no entities return an error
        quote! {
            Err(::mimic::orm::OrmError::EntityNotFound(query.path.to_string()).into())
        }
    } else {
        // Otherwise, generate match arms dynamically
        let mut load_entities = quote!();
        for (entity_path, _) in entities {
            let generic: Path = parse_str(&entity_path).unwrap();

            load_entities.extend(quote! {
                #entity_path => {
                    let executor = ::mimic::query::LoadExecutor::<#generic>::new(query.clone());
                    executor.response(&DB)
                }
            });
        }

        quote! {
            let path = &query.path;
            let res = match path.as_str() {
                #load_entities
                _ => Err(::mimic::orm::OrmError::EntityNotFound(path.to_string()).into())
            }?;

            Ok(res)
        }
    };

    let q = quote! {
        #[::mimic::ic::query]
        pub fn query_load(
            query: ::mimic::query::LoadQuery,
        ) -> Result<::mimic::query::LoadResponse, ::mimic::Error> {
            #inner
        }
    };

    builder.extend(q);
}

//
// query_save
//

fn query_save(builder: &mut ActorBuilder) {
    let entities = builder.get_entities();

    let inner = if entities.is_empty() {
        // if there are no entities return an error
        quote! {
            Err(::mimic::orm::OrmError::EntityNotFound(query.path.to_string()).into())
        }
    } else {
        // Otherwise, generate match arms dynamically
        let mut save_entities = quote!();
        for (entity_path, _) in entities {
            let generic: Path = parse_str(&entity_path).unwrap();

            save_entities.extend(quote! {
                #entity_path => executor.execute::<#generic>(&DB),
            });
        }

        quote! {
            let executor = ::mimic::query::SaveExecutor::new(query.clone());
            let path = &query.path;
            let res = match path.as_str() {
                #save_entities
                _ => Err(::mimic::orm::OrmError::EntityNotFound(path.to_string()).into())
            }?;

            Ok(res)
        }
    };

    let q = quote! {
        #[::mimic::ic::update]
        pub fn query_save(
            query: ::mimic::query::SaveQuery
        ) -> Result<::mimic::query::SaveResponse, ::mimic::Error> {
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
