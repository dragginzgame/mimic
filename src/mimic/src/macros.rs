///
/// MACROS
///

// mimic_start
// macro to be included at the start of each canister lib.rs file
#[macro_export]
macro_rules! mimic_start {
    ($config:expr) => {
        // actor.rs
        include!(concat!(env!("OUT_DIR"), "/actor.rs"));

        // mimic_init
        fn mimic_init() {
            // schema
            let schema_json = include_str!(concat!(env!("OUT_DIR"), "/schema.rs"));
            ::mimic::schema::state::init_schema_json(schema_json).unwrap();

            // config
            let toml = include_str!($config);
            ::mimic::config::init_config_toml(toml).unwrap();

            // fixtures
            init_fixtures().unwrap();
        }
    };
}

// mimic_memory_manager
#[macro_export]
macro_rules! mimic_memory_manager {
    () => {
        thread_local! {

            ///
            /// Define MEMORY_MANAGER thread-locally for the entire scope
            ///
            pub static MEMORY_MANAGER: ::std::cell::RefCell<
                ::mimic::ic::structures::memory::MemoryManager<
                    ::mimic::ic::structures::DefaultMemoryImpl,
                >,
            > = ::std::cell::RefCell::new(::mimic::ic::structures::memory::MemoryManager::init(
                ::mimic::ic::structures::DefaultMemoryImpl::default(),
            ));

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
        ::ic_cdk::export_candid!();
    };
}
