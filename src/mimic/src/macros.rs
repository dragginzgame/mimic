// mimic_start
// macro to be included at the start of each canister lib.rs file
#[macro_export]
macro_rules! mimic_start {
    () => {
        // actor.rs
        include!(concat!(env!("OUT_DIR"), "/actor.rs"));

        // mimic_init
        fn mimic_init() {
            // schema
            let schema_json = include_str!(concat!(env!("OUT_DIR"), "/schema.rs"));
            ::mimic::schema::state::init_schema_json(schema_json).unwrap();

            // fixtures
            mimic_init_fixtures().unwrap();
        }
    };
}
