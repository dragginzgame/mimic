use crate::actor::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;

// generate
#[must_use]
pub fn generate(_builder: &ActorBuilder) -> TokenStream {
    quote! {
        #[::mimic::export::icu::cdk::query]
        pub fn mimic_metrics(select: ::mimic::metrics::MetricsSelect) -> Result<::mimic::metrics::MetricsReport, ::mimic::Error> {
            Ok(::mimic::interface::metrics::metrics_report(&db(), select))
        }

        #[::mimic::export::icu::cdk::update]
        pub fn mimic_metrics_reset() -> Result<(), ::mimic::Error> {
            ::mimic::interface::metrics::metrics_reset();
            Ok(())
        }
    }
}
