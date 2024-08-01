#[macro_export]
macro_rules! actor_start {
    ($config_path:expr) => {{
        let config_str = include_str!("../../../config.toml");
        ::mimic::config::init_toml(config_str).expect("Failed to load configuration");

        panic!("{}", ::mimic::config::get_config().unwrap());

        include!(concat!("../../../../../generated/actor/", $actor, ".rs"));
    }};
}

// actor_end
#[macro_export]
macro_rules! actor_end {
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
