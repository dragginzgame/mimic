use crate::actor::ActorBuilder;
use mimic_schema::types::StoreType;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

// generate
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    stores(builder)
}

// stores
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
                static #cell_ident: ::std::cell::RefCell<::mimic::db::store::IndexStore> =
                    ::std::cell::RefCell::new(::icu::icu_register_memory!(
                        ::mimic::db::store::IndexStore,
                        #memory_id,
                        ::mimic::db::store::IndexStore::init
                    ));
            });

            index_inits.extend(quote! {
                index_registry.register(#store_path_lit, &#cell_ident);
            });
        } else {
            // Data store
            data_defs.extend(quote! {
                static #cell_ident: ::std::cell::RefCell<::mimic::db::store::DataStore> =
                    ::std::cell::RefCell::new(::icu::icu_register_memory!(
                        ::mimic::db::store::DataStore,
                        #memory_id,
                        ::mimic::db::store::DataStore::init
                    ));
            });

            data_inits.extend(quote! {
                data_registry.register(#store_path_lit, &#cell_ident);
            });
        }
    }

    let data_registry = wrap_registry_init("data_registry", data_inits);
    let index_registry = wrap_registry_init("index_registry", index_inits);

    quote! {
        thread_local! {
            #data_defs
            #index_defs

            static DATA_REGISTRY: ::std::rc::Rc<::mimic::db::store::StoreRegistry<::mimic::db::store::DataStore>> =
                ::std::rc::Rc::new(#data_registry);

            static INDEX_REGISTRY: ::std::rc::Rc<::mimic::db::store::StoreRegistry<::mimic::db::store::IndexStore>> =
                ::std::rc::Rc::new(#index_registry);

        }
    }
}

// wrap_registry_init
fn wrap_registry_init(name: &str, inits: TokenStream) -> TokenStream {
    if inits.is_empty() {
        quote! {
            ::mimic::db::store::StoreRegistry::new()
        }
    } else {
        let name_ident = format_ident!("{}", name);
        quote! {
            {
                let mut #name_ident = ::mimic::db::store::StoreRegistry::new();
                #inits
                #name_ident
            }
        }
    }
}
