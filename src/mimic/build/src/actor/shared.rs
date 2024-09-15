use super::ActorBuilder;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    let q = quote! {
        // user needs to implement the StartupHooks trait
        pub struct StartupManager {}
    };

    builder.extend_actor(q);
}
