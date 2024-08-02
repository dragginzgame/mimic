// mimic_start
// macro to be included at the start of each canister lib.rs file
#[macro_export]
macro_rules! mimic_start {
    ($config_path:expr) => {{
        let config_str = include_str!("../../../config.toml");
        ::mimic::config::init_toml(config_str).expect("Failed to load configuration");

        panic!("{}", ::mimic::config::get_config().unwrap());

        include!(concat!("../../../../../generated/actor/", $actor, ".rs"));
    }};
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
