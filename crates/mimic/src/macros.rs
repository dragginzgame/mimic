// mimic_start
// macro to be included at the start of each canister lib.rs file
#[macro_export]
macro_rules! mimic_start {
    () => {
        // actor.rs
        include!(concat!(env!("OUT_DIR"), "/actor.rs"));

        fn mimic_init() {
            // fixtures
            mimic_init_fixtures().unwrap();
        }
    };
}
