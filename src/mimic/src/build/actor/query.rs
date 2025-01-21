use super::ActorBuilder;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    query(builder);
}

// query
fn query(builder: &mut ActorBuilder) {
    let q = quote! {
        pub fn query_load(
            query: ::mimic::db::query::LoadQueryDyn,
            path: &str,
        ) -> Result<::mimic::db::query::RowIteratorDyn, ::mimic::db::query::Error> {
            let executor = ::mimic::db::query::LoadExecutorDyn::new(query, path);
            let res = DB.with(|db| executor.execute(db))?;

            Ok(res)
        }
    };

    builder.extend_actor(q);
}
