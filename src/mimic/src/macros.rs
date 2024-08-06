///
/// MACROS
///

// mimic_start
// macro to be included at the start of each canister lib.rs file
#[macro_export]
macro_rules! mimic_start {
    ($actor:expr) => {
        include!(concat!("../../../../generated/actor/", $actor, ".rs"));

        // startup
        // code called on all canister startups (install, upgrade)
        fn startup() -> Result<(), Error> {
            let config_str = include_str!("../../../config.toml");
            ::mimic::config::init_config_toml(config_str).map_err(::mimic::Error::from)?;

            let schema_json = include_str!("../../../../generated/schema/schema.json");
            ::mimic::core::schema::init_schema_json(schema_json).map_err(::mimic::Error::from)?;

            startup2()
        }
    };
}

// mimic_end
// macro that needs to be included as the last item in the actor lib.rs file
#[macro_export]
macro_rules! mimic_end {
    () => {
        // export_candid
        // has to be at the end
        ::mimic::ic::export_candid!();
    };
}

// perf
#[macro_export]
macro_rules! perf {
    () => {
        ::mimic::api::defer!(::mimic::ic::log!(
            Log::Perf,
            "api call used {} instructions ({})",
            ::mimic::ic::api::performance_counter(1),
            module_path!()
        ));
    };
}
