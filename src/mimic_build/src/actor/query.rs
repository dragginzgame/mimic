use crate::actor::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;

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
    /*
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
    */
}
