use super::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, Path};

// extend
pub fn extend(builder: &mut ActorBuilder) {
    let mut q = quote!();

    q.extend(guard_crud(builder));
    q.extend(crud_load(builder));
    q.extend(crud_save(builder));
    q.extend(crud_delete(builder));

    builder.extend_actor(q);
}

//
// guard_crud
// uses the schema to see what permission is required
//
#[must_use]
pub fn guard_crud(_: &ActorBuilder) -> TokenStream {
    quote! {
        async fn guard_crud(entity: &str, action: ::mimic::orm::schema::types::CrudAction) -> Result<(), ::mimic::api::Error> {
            // are there crud permissions?
            let crud = ::mimic::api::crud::get_entity(entity)?;

            // check permission action
            let policy = match action {
                ::mimic::orm::schema::types::CrudAction::Load => crud.load.clone(),
                ::mimic::orm::schema::types::CrudAction::Save => crud.save.clone(),
                ::mimic::orm::schema::types::CrudAction::Delete => crud.delete.clone(),
            };

            guard(
                vec![Guard::Policy(policy)]
            )
            .await?;

            Ok(())
        }
    }
}

// crud_load
#[must_use]
pub fn crud_load(builder: &ActorBuilder) -> TokenStream {
    // fetch every entity for this canister
    let mut calls = Vec::new();
    for (path, _) in builder.get_entities() {
        let path_ident: Path = parse_str(&path).unwrap();

        calls.push(quote! {
            #path => ::mimic::api::crud::load::<#path_ident>(db, request),
        });
    }

    quote! {
        #[::mimic::ic::query(composite = true)]
        #[allow(clippy::too_many_lines)]
        #[allow(clippy::match_single_binding)]
        #[allow(unused_variables)]
        async fn crud_load(request: ::mimic::db::query::LoadRequest) -> Result<::mimic::db::query::LoadResponse, ::mimic::api::Error> {
            guard_crud(&request.entity, ::mimic::orm::schema::types::CrudAction::Load).await?;

            let res = DB.with(|db| {
                match request.entity.as_str() {
                    #(#calls)*
                    _ => Err(::mimic::api::crud::Error::entity_not_found(&request.entity))
                }
            }).map_err(::mimic::api::Error::from)?;

            Ok(res)
        }
    }
}

// crud_save
#[must_use]
pub fn crud_save(builder: &ActorBuilder) -> TokenStream {
    let mut calls = Vec::new();
    for (path, _) in builder.get_entities() {
        let path_ident: Path = parse_str(&path).unwrap();

        calls.push(quote! {
            #path => ::mimic::api::crud::save::<#path_ident>(db, &request),
        });
    }

    quote! {
        #[::mimic::ic::update]
        #[allow(clippy::too_many_lines)]
        #[allow(clippy::match_single_binding)]
        #[allow(unused_variables)]
        async fn crud_save(request: ::mimic::db::query::SaveRequest) -> Result<::mimic::db::query::SaveResponse, ::mimic::api::Error> {
            guard_crud(&request.entity, ::mimic::orm::schema::types::CrudAction::Save).await?;

            let res = DB.with(|db| {
                match request.entity.as_str() {
                    #(#calls)*
                    _ => Err(::mimic::api::crud::Error::entity_not_found(&request.entity))
                }
            }).map_err(::mimic::api::Error::from)?;

            Ok(res)
        }
    }
}

// crud_delete
#[must_use]
pub fn crud_delete(builder: &ActorBuilder) -> TokenStream {
    let mut calls = Vec::new();
    for (path, _) in builder.get_entities() {
        let path_ident: Path = parse_str(&path).unwrap();

        calls.push(quote! {
            #path => ::mimic::api::crud::delete::<#path_ident>(db, &request),
        });
    }

    quote! {
        #[::mimic::ic::update]
        #[allow(clippy::too_many_lines)]
        #[allow(clippy::match_single_binding)]
        #[allow(unused_variables)]
        async fn crud_delete(request: ::mimic::db::query::DeleteRequest) -> Result<::mimic::db::query::DeleteResponse, ::mimic::api::Error> {
            guard_crud(&request.entity, ::mimic::orm::schema::types::CrudAction::Delete).await?;

            let res = DB.with(|db| {
                match request.entity.as_str() {
                    #(#calls)*
                    _ => Err(::mimic::api::crud::Error::entity_not_found(&request.entity))
                }
            })?;

            Ok(res)
        }
    }
}
