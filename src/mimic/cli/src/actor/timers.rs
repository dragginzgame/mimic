use super::ActorBuilder;
use quote::quote;

// extend
pub fn extend(builder: &mut ActorBuilder) {
    let q = quote! {
        pub fn init_timers() -> Result<(), ::mimic::Error> {

            //
            // check_cycles
            //
            //::ic::println!("init_timers: NO TIMERS RIGHT NOW");
            //let secs = ::std::time::Duration::from_secs(600);



            Ok(())
        }
    };

    // code
    builder.add_init_hook("actorgen::init_timers");
    builder.extend_module(q);
}
