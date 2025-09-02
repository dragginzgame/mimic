use crate::actor::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;

// generate
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    // Build (ENTITY_ID, PATH) mapping for all entities
    let mut pairs: Vec<TokenStream> = Vec::new();
    for (entity_path, _) in builder.get_entities() {
        let entity_ident: syn::Path = syn::parse_str(&entity_path)
            .unwrap_or_else(|_| panic!("invalid entity path: {entity_path}"));
        pairs.push(quote! { (#entity_ident::ENTITY_ID, #entity_ident::PATH) });
    }

    quote! {
        const MIMIC_ENTITY_ID_PATH: &[(u64, &str)] = &[
            #(#pairs,)*
        ];

        /// Storage snapshot (live view).
        /// Includes data/index store stats and per-entity breakdown by store.
        #[::mimic::export::icu::cdk::query]
        pub fn mimic_storage() -> Result<::mimic::metrics::StorageReport, ::mimic::Error> {
            Ok(::mimic::interface::storage::storage_report(&db(), MIMIC_ENTITY_ID_PATH))
        }

        /// Ephemeral metrics since `since_ms` (counters + per-entity summaries).
        #[::mimic::export::icu::cdk::query]
        pub fn mimic_metrics() -> Result<::mimic::metrics::MetricsReport, ::mimic::Error> {
            Ok(::mimic::interface::metrics::metrics_report(&db()))
        }

        /// Reset ephemeral metrics and refresh `since_ms`.
        #[::mimic::export::icu::cdk::update]
        pub fn mimic_metrics_reset() -> Result<(), ::mimic::Error> {
            ::mimic::interface::metrics::metrics_reset();
            Ok(())
        }
    }
}
