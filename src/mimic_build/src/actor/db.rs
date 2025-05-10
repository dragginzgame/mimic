use crate::actor::ActorBuilder;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

// generate
#[must_use]
pub fn generate(builder: &ActorBuilder) -> TokenStream {
    stores(builder)
}

// stores
fn stores(builder: &ActorBuilder) -> TokenStream {
    let mut store_defs = quote!();
    let mut db_inserts = quote!();

    for (store_path, store) in builder.get_stores() {
        let cell_ident = format_ident!("{}", &store.ident);
        let memory_id = store.memory_id;

        // define each store statically within the thread_local! macro
        store_defs.extend(quote! {
            static #cell_ident: ::std::cell::RefCell<::mimic::db::Store> = ::std::cell::RefCell::new(
                ::mimic::db::Store::init(
                    MEMORY_MANAGER.with_borrow(|mm| mm.get(
                        ::mimic::ic::structures::memory::MemoryId::new(#memory_id)
                    ))
                )
            );
        });

        // Prepare insertions into the Db
        db_inserts.extend(quote! {
            db.insert_store(#store_path, &#cell_ident);
        });
    }

    // format stores variable
    let db = if db_inserts.is_empty() {
        quote! {
            ::mimic::db::Db::new()
        }
    } else {
        quote! {
            {
                let mut db = ::mimic::db::Db::new();
                #db_inserts

                db
            }
        }
    };

    // combine everything into a thread_local! macro and additional functions
    quote! {
        thread_local! {
            #store_defs

            static DB: ::std::rc::Rc<::mimic::db::Db> = ::std::rc::Rc::new(#db);
        }
    }
}
