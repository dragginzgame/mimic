use super::ActorBuilder;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    stores(builder);
}

// stores
fn stores(builder: &mut ActorBuilder) {
    let mut store_defs = quote!();
    let mut db_inserts = quote!();

    for (store_path, store) in builder.get_stores() {
        let cell_ident = store.cell_ident();
        let memory_id = store.memory_id;

        // define each store statically within the thread_local! macro
        store_defs.extend(quote! {
            static #cell_ident: RefCell<::mimic::db::Store> = RefCell::new(
                ::mimic::db::Store::init(
                    ::mimic::core::state::MEMORY_MANAGER.with(|mm| mm.borrow().get(
                        ::mimic::ic::structures::memory::MemoryId::new(#memory_id)
                    ))
                )
            );
        });

        // Prepare insertions into the Db
        db_inserts.extend(quote! {
            db.insert(#store_path, & #cell_ident);
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
                let mut db =::mimic::db::Db::new();
                #db_inserts
                db
            }
        }
    };

    // combine everything into a thread_local! macro and additional functions
    let q = quote! {
        thread_local! {
            #store_defs

            static DB: ::std::sync::Arc<::mimic::db::Db> = ::std::sync::Arc::new(#db);
        }
    };

    builder.extend_actor(q);
}
