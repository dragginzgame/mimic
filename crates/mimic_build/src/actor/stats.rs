use crate::actor::ActorBuilder;
use proc_macro2::TokenStream;
use quote::quote;

// generate
#[must_use]
pub fn generate(_builder: &ActorBuilder) -> TokenStream {
    quote! {
        #[::mimic::export::icu::cdk::query]
        pub fn mimic_stats() -> Result<::mimic::interface::stats::DbStats, ::mimic::Error> {
            use ::mimic::interface::stats::{DbStats, IndexStats, StoreStats};

            let mut data = Vec::new();
            let mut index = Vec::new();

            db().with_data(|reg| {
                reg.for_each(|path, store| {
                    data.push(StoreStats {
                        path: path.to_string(),
                        entries: store.len() as u64,
                        min_key: store.first_key_value().map(|(k, _)| k.into()),
                        max_key: store.last_key_value().map(|(k, _)| k.into()),
                        memory_bytes: store.memory_bytes(),
                    });
                });
            });

            db().with_index(|reg| {
                reg.for_each(|path, store| {
                    index.push(IndexStats {
                        path: path.to_string(),
                        entries: store.len() as u64,
               memory_bytes: store.memory_bytes(),
                    });
                });
            });

            Ok(DbStats {
                data_stores: data,
                index_stores: index,
            })
        }
    }
}
