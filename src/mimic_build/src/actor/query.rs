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
                    let query = ::mimic::query::load::<#generic>().query(query);

                    query.response(&DB)
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
        #[allow(unused_variables)]
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
            Err(::mimic::interface::query::QueryError::EntityNotFound(path))
                .map_err(::mimic::interface::InterfaceError::from)?
        }
    } else {
        // Otherwise, generate match arms dynamically
        let mut save_entities = quote!();
        for (entity_path, _) in entities {
            let generic: Path = parse_str(&entity_path).unwrap();

            save_entities.extend(quote! {
                #entity_path => {
                    let query = ::mimic::query::save().query::<#generic>(query)?;

                    query.execute(&DB)
                }
            });
        }

        quote! {
            let res = match path.as_str() {
                #save_entities

                _ => return Err(::mimic::interface::query::QueryError::EntityNotFound(path))
                    .map_err(::mimic::interface::InterfaceError::from)?
            }?;

            Ok(res)
        }
    };

    quote! {
        #[::mimic::ic::update]
        #[allow(unused_variables)]
        pub fn mimic_query_save(
            path: String,
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
        #[allow(unused_variables)]
        pub fn mimic_query_delete(
            query: ::mimic::query::DeleteQuery,
        ) -> Result<::mimic::query::DeleteResponse, ::mimic::Error> {
            let query = ::mimic::query::delete().query(query);

            query.execute(&DB)
        }
    }
}
