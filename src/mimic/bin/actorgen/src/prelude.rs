use crate::ActorBuilder;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    let q = quote! {

        //
        // Prelude
        //
        // NOTE: Do not put the candid macros (query, update etc.) directly within this prelude as the endpoints
        // will fail to be registered with the export_candid! macro
        //

        pub mod prelude {
            pub use ::candid::Principal;
            pub use ::lib_ulid::Ulid;
            pub use ::mimic::{
                api::{
                    perf,
                    auth::{guard, Guard},
                    request::{Request, RequestKind, Response},
                },
                core::state::{
                    AppCommand, AppState, AppStateManager, CanisterState, CanisterStateManager,
                    SubnetIndex, SubnetIndexManager, User, UserIndex, UserIndexManager,
                },
                ic::{caller, format_cycles, id, log, Log},
                orm::traits::EntityFixture,
            };
            pub use ::std::cell::RefCell;
        }

        pub use prelude::*;
    };

    // extend
    builder.extend_actor(q);
}
