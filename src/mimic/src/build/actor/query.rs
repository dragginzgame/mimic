use super::ActorBuilder;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    query_load(builder);
    query_load_response(builder);
}

// query_load
fn query_load(builder: &mut ActorBuilder) {
    let q = quote! {
        pub async fn query_canister(
            canister_path: &str,
            query: ::mimic::db::query::LoadQueryDyn,
        ) -> Result<Vec<::mimic::db::types::DataRow>, ::mimic::api::Error> {
            // look up canister_id
            let cid = ::mimic::core::state::SubnetIndexManager::try_get_canister(canister_path)?;

            // do the call
            let res = ::mimic::api::ic::call::call::<
                _,
                (Result<Vec<::mimic::db::types::DataRow>, ::mimic::api::Error>,),
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
        pub fn query_load(
            query: ::mimic::db::query::LoadQueryDyn,
        ) -> Result<Vec<::mimic::db::types::DataRow>, ::mimic::api::Error> {
            let executor = ::mimic::db::query::LoadExecutorDyn::new(query);

            let res: Result<Vec<::mimic::db::types::DataRow>, ::mimic::api::Error> = DB.with(|db| {
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
