use crate::ActorBuilder;
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
            #(#pairs),*
        ];

        /// Storage snapshot (live view).
        /// Includes data/index store stats and per-entity breakdown by store.
        #[::mimic::export::canic::cdk::query]
        pub fn mimic_snapshot() -> Result<::mimic::obs::snapshot::StorageReport, ::mimic::Error> {
            Ok(::mimic::obs::snapshot::storage_report(&DB, MIMIC_ENTITY_ID_PATH))
        }

        /// Runtime event logs (oldest → newest). Ephemeral, in-memory buffer.
        #[::mimic::export::canic::cdk::query]
        pub fn mimic_logs() -> Result<Vec<String>, ::mimic::Error> {
            Ok(::mimic::obs::log::logs_snapshot())
        }

        /// Ephemeral event report since the internal `since_ms` (counters + per-entity summaries).
        /// Call `mimic_metrics_reset` to reset counters and refresh `since_ms`.
        #[::mimic::export::canic::cdk::query]
        pub fn mimic_metrics() -> Result<::mimic::obs::metrics::EventReport, ::mimic::Error> {
            Ok(::mimic::obs::metrics::report())
        }

        /// Reset ephemeral event state and refresh `since_ms`.
        #[::mimic::export::canic::cdk::update]
        pub fn mimic_metrics_reset() -> Result<(), ::mimic::Error> {
            ::mimic::obs::metrics::reset_all();

            Ok(())
        }

    }
}
