use crate::build::actor::ActorBuilder;
use mimic::schema::types::StoreType;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

// generate
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    stores(builder)
}

// stores
fn stores(builder: &ActorBuilder) -> TokenStream {
    let mut data_store_defs = quote!();
    let mut index_store_defs = quote!();

    let mut data_store_inserts = quote!();
    let mut index_store_inserts = quote!();

    for (store_path, store) in builder.get_stores() {
        let cell_ident = format_ident!("{}", &store.ident);
        let memory_id = store.memory_id;
        let store_path_lit = store_path;

        if matches!(store.ty, StoreType::Index) {
            // Index store
            index_store_defs.extend(quote! {
                static #cell_ident: ::std::cell::RefCell<::mimic::db::IndexStore> =
                    ::std::cell::RefCell::new(::icu::icu_register_memory!(
                        ::mimic::db::IndexStore,
                        #memory_id,
                        ::mimic::db::IndexStore::init
                    ));
            });

            index_store_inserts.extend(quote! {
                index_registry.register(#store_path_lit, &#cell_ident);
            });
        } else {
            // Data store
            data_store_defs.extend(quote! {
                static #cell_ident: ::std::cell::RefCell<::mimic::db::DataStore> =
                    ::std::cell::RefCell::new(::icu::icu_register_memory!(
                        ::mimic::db::DataStore,
                        #memory_id,
                        ::mimic::db::DataStore::init
                    ));
            });

            data_store_inserts.extend(quote! {
                data_registry.register(#store_path_lit, &#cell_ident);
            });
        }
    }

    let data_registry = if data_store_inserts.is_empty() {
        quote! {
            ::mimic::db::StoreRegistry::new()
        }
    } else {
        quote! {
            {
                let mut data_registry = ::mimic::db::StoreRegistry::new();
                #data_store_inserts

                data_registry
            }
        }
    };

    let index_registry = if index_store_inserts.is_empty() {
        quote! {
            ::mimic::db::StoreRegistry::new()
        }
    } else {
        quote! {
            {
                let mut index_registry = ::mimic::db::StoreRegistry::new();
                #index_store_inserts

                index_registry
            }
        }
    };

    quote! {
        thread_local! {
            #data_store_defs
            #index_store_defs

            static DB: ::std::rc::Rc<::mimic::db::StoreRegistry<::mimic::db::DataStore>> =
                ::std::rc::Rc::new(#data_registry);

            static INDEXES: ::std::rc::Rc<::mimic::db::StoreRegistry<::mimic::db::IndexStore>> =
                ::std::rc::Rc::new(#index_registry);
        }
    }
}
