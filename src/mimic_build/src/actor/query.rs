use crate::actor::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path, parse_str};

// generate
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    let mut tokens = quote!();

    tokens.extend(mimic_query_load(builder));
    tokens.extend(mimic_query_save(builder));
    tokens.extend(mimic_query_delete(builder));

    tokens
}

// mimic_query_load
fn mimic_query_load(builder: &ActorBuilder) -> TokenStream {
    let entities = builder.get_entities();

    let inner = if entities.is_empty() {
        // if there are no entities return an error
        quote! {
            Err(::mimic::interface::query::QueryError::EntityNotFound(path))
                .map_err(::mimic::interface::InterfaceError::from)?
        }
    } else {
        // Otherwise, generate match arms dynamically
        let mut load_entities = quote!();
        for (entity_path, _) in entities {
            let generic: Path = parse_str(&entity_path).unwrap();

            load_entities.extend(quote! {
                #entity_path => {
                    let builder = ::mimic::query::LoadQueryBuilder::<#generic>::new(query.clone());
                    builder.response(&DB)
                }
            });
        }

        quote! {
            let res = match path.as_str() {
                #load_entities

                _ => return Err(::mimic::interface::query::QueryError::EntityNotFound(path))
                    .map_err(::mimic::interface::InterfaceError::from)?
            }?;

            Ok(res)
        }
    };

    quote! {
        #[::mimic::ic::query]
        pub fn mimic_query_load(
            path: String,
            query: ::mimic::query::LoadQuery,
        ) -> Result<::mimic::query::LoadResponse, ::mimic::Error> {
            #inner
        }
    }
}

// mimic_query_save
fn mimic_query_save(builder: &ActorBuilder) -> TokenStream {
    let entities = builder.get_entities();

    let inner = if entities.is_empty() {
        // if there are no entities return an error
        quote! {
            Err(::mimic::interface::query::QueryError::EntityNotFound(query.path.to_string()))
                .map_err(::mimic::interface::InterfaceError::from)?
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

                _ => return Err(::mimic::interface::query::QueryError::EntityNotFound(query.path.to_string()))
                    .map_err(::mimic::interface::InterfaceError::from)?
            }?;

            Ok(res)
        }
    };

    quote! {
        #[::mimic::ic::update]
        pub fn mimic_query_save(
            query: ::mimic::query::SaveQuery
        ) -> Result<::mimic::query::SaveResponse, ::mimic::Error> {
            #inner
        }
    }
}

// mimic_query_delete
fn mimic_query_delete(_builder: &ActorBuilder) -> TokenStream {
    quote! {
        #[::mimic::ic::update]
        pub fn mimic_query_delete(
            query: ::mimic::query::DeleteQueryDyn,
        ) -> Result<::mimic::query::DeleteResponse, ::mimic::Error> {
            let query = ::mimic::query::delete_dyn().query(query);

            query.execute(&DB)
        }
    }
}

/*


// generate
#[must_use]
pub fn generate(_builder: &ActorBuilder) -> TokenStream {
    quote! {
        // load
        #[::mimic::ic::query]
        pub fn mimic_query_load(
            query: ::mimic::query::LoadQuery,
        ) -> Result<::mimic::query::LoadResponse, ::mimic::Error> {
            ::mimic::query::load_dyn()
                .query(query)
                .response(&DB)
        }
    }

    // save
        #[::mimic::ic::update]
        pub fn mimic_query_save(
            query: ::mimic::query::SaveQuery
        ) -> Result<::mimic::query::SaveResponse, ::mimic::Error> {
            let executor = ::mimic::query::SaveQueryDynExecutor::new(query);

            executor.response(&DB)
        }

        // delete
        #[::mimic::ic::update]
        pub fn mimic_query_delete(
            query: ::mimic::query::DeleteQuery,
        ) -> Result<::mimic::query::DeleteResponse, ::mimic::Error> {
            let executor = ::mimic::query::DeleteExecutor::new(query);

            executor.execute(&DB)
        }
    }
}
    */
