use crate::ActorBuilder;
use mimic_schema::types::StoreType;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse_str;

///
/// Entry point for codegen.
/// Expands into:
/// - `init_memory()` to reserve ICU memory range
/// - thread-local Data/Index store definitions
/// - registry wiring
/// - a global `db()` accessor
///
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    let mut tokens = quote!();

    // Generate memory reservation function
    tokens.extend(reserve_memory(builder));

    // Generate store definitions + registries
    tokens.extend(stores(builder));

    tokens
}

///
/// Emit a function that reserves this canister’s memory range
/// with the ICU MemoryRegistry. Must be called once in
/// `canister_init` / `post_upgrade`.
///
fn reserve_memory(builder: &ActorBuilder) -> TokenStream {
    let canister = &builder.canister;
    let canister_path = canister.def.path();
    let memory_min = canister.memory_min;
    let memory_max = canister.memory_max;

    quote! {
        /// Must be called once during canister init or upgrade
        pub fn mimic_reserve_memory() {
            ::icu::memory::MemoryRegistry::reserve_range(
                #memory_min,
                #memory_max,
                stringify!(#canister_path),
            ).expect("failed to reserve canister memory range");
        }
    }
}

///
/// Generate thread-local DataStores and IndexStores for this canister,
/// along with their registries and the `db()` accessor.
///
fn stores(builder: &ActorBuilder) -> TokenStream {
    let mut data_defs = quote!();
    let mut index_defs = quote!();
    let mut data_inits = quote!();
    let mut index_inits = quote!();

    for (store_path, store) in builder.get_stores() {
        let cell_ident = format_ident!("{}", &store.ident);
        let memory_id = store.memory_id;
        let store_path_lit = store_path;

        if matches!(store.ty, StoreType::Index) {
            // Index store
            index_defs.extend(quote! {
                ::icu::thread_local_memory! {
                    static #cell_ident: ::std::cell::RefCell<::mimic::db::store::IndexStore> =
                        ::std::cell::RefCell::new(::mimic::db::store::IndexStore::init(
                            ::icu::icu_memory!(IndexStore, #memory_id)
                        ));
                }
            });

            index_inits.extend(quote! {
                reg.register(#store_path_lit, &#cell_ident);
            });
        } else {
            // Data store
            data_defs.extend(quote! {
                ::icu::thread_local_memory! {
                    static #cell_ident: ::std::cell::RefCell<::mimic::db::store::DataStore> =
                        ::std::cell::RefCell::new(::mimic::db::store::DataStore::init(
                            ::icu::icu_memory!(DataStore, #memory_id)
                        ));
                }
            });

            data_inits.extend(quote! {
                reg.register(#store_path_lit, &#cell_ident);
            });
        }
    }

    let canister_path: syn::Path = parse_str(&builder.canister.def.path())
        .unwrap_or_else(|_| panic!("invalid canister path: {}", builder.canister.def.path()));

    quote! {
        #data_defs
        #index_defs

        thread_local! {
            // registries
            #[allow(unused_mut)]
            static DATA_REGISTRY: ::mimic::db::store::DataStoreRegistry = {
                let mut reg = ::mimic::db::store::DataStoreRegistry::new();
                #data_inits
                reg
            };

            #[allow(unused_mut)]
            static INDEX_REGISTRY: ::mimic::db::store::IndexStoreRegistry = {
                let mut reg = ::mimic::db::store::IndexStoreRegistry::new();
                #index_inits
                reg
            };
        }

        /// Global accessor (fat handle) for this canister’s DB
        #[must_use]
        pub const fn db() -> ::mimic::db::Db<#canister_path> {
            ::mimic::db::Db::new(&DATA_REGISTRY, &INDEX_REGISTRY)
        }
    }
}
