use super::ActorBuilder;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    query_load(builder);
    query_load_response(builder);

    query_delete(builder);
    query_delete_response(builder);
}

//
// LOAD
//

// query_load
fn query_load(builder: &mut ActorBuilder) {
    let q = quote! {
        pub async fn query_load(
            canister_path: &str,
            query: ::mimic::query::LoadQuery,
        ) -> Result<Vec<::mimic::db::types::DataRow>, ::mimic::api::ApiError> {
            // look up canister_id
            let cid = ::mimic::core::state::SubnetIndexManager::try_get_canister(canister_path)?;

            // do the call
            let res = ::mimic::api::ic::call::call::<
                _,
                (Result<Vec<::mimic::db::types::DataRow>, ::mimic::api::ApiError>,),
            >(cid, "query_load_response", (query,))
            .await?
            .0?;

            Ok(res)
        }
    };

    builder.extend_module(q);
}

// query_load_response
fn query_load_response(builder: &mut ActorBuilder) {
    let q = quote! {
        #[::mimic::ic::query]
        pub fn query_load_response(
            query: ::mimic::query::LoadQuery,
        ) -> Result<Vec<::mimic::db::types::DataRow>, ::mimic::api::ApiError> {
            let executor = ::mimic::query::LoadExecutor::new(query);
            let res: Result<Vec<::mimic::db::types::DataRow>, ::mimic::api::ApiError> = DB.with(|db| {
                let res = executor.execute(db)?;
                let rows = res.data_rows().collect();

                Ok(rows)
            });
            let res = res?;

            Ok(res)
        }
    };

    builder.extend_actor(q);
}

//
// DELETE
//

// query_delete
fn query_delete(builder: &mut ActorBuilder) {
    let q = quote! {
        pub async fn query_delete(
            canister_path: &str,
            query: ::mimic::query::DeleteQuery,
        ) -> Result<::mimic::query::DeleteResponse, ::mimic::api::ApiError> {
            // look up canister_id
            let cid = ::mimic::core::state::SubnetIndexManager::try_get_canister(canister_path)?;

            // do the call
            let res = ::mimic::api::ic::call::call::<
                _,
                (Result<::mimic::query::DeleteResponse, ::mimic::api::ApiError>,),
            >(cid, "query_delete_response", (query,))
            .await?
            .0?;

            Ok(res)
        }
    };

    builder.extend_module(q);
}

// query_delete_response
fn query_delete_response(builder: &mut ActorBuilder) {
    let q = quote! {
        #[::mimic::ic::update]
        pub fn query_delete_response(
            query: ::mimic::query::DeleteQuery,
        ) -> Result<::mimic::query::DeleteResponse, ::mimic::api::ApiError> {
            let executor = ::mimic::query::DeleteExecutor::new(query);
            let res: Result<::mimic::query::DeleteResponse, ::mimic::api::ApiError> = DB.with(|db| {
                let res = executor.execute(db)?;

                Ok(res)
            });
            let res = res?;

            Ok(res)
        }
    };

    builder.extend_actor(q);
}
