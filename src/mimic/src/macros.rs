///
/// MACROS
///

// mimic_build
// for the various build.rs files
#[macro_export]
macro_rules! mimic_build {
    ($actor:expr) => {
        use std::{fs::File, io::Write, path::PathBuf};

        //
        // cargo directives
        //

        // Retrieve the target triple from the environment
        let target = std::env::var("TARGET").unwrap();

        // all
        println!("cargo:rerun-if-changed=build.rs");

        // macOS linker
        if target.contains("apple") {
            println!("cargo:rustc-link-arg=-Wl,-all_load");
            println!("cargo:rustc-flags=-C opt-level=0");
        }

        // Get the output directory set by Cargo
        let out_dir = ::std::env::var("OUT_DIR").expect("OUT_DIR not set");

        // build schema
        let output = ::mimic::build::schema::schema().unwrap();

        // write schema
        let schema_file = PathBuf::from(out_dir).join("schema.rs");
        let mut file = File::create(schema_file)?;
        file.write_all(output.as_bytes())?;
    };
}

// mimic_start
// macro to be included at the start of each canister lib.rs file
#[macro_export]
macro_rules! mimic_start {
    ($config:expr) => {
        #[::mimic::ic::init]
        fn init() {
            _init().unwrap();
        }

        // _init
        // code called on all canister startups (install, upgrade)
        fn _init() -> Result<(), ::mimic::DynError> {
            // schema
            let schema_json = include_str!(concat!(env!("OUT_DIR"), "/schema.rs"));
            ::mimic::core::schema::init_schema_json(schema_json)?;

            // config
            let toml = include_str!($config);
            ::mimic::core::config::init_config_toml(toml)?;

            Ok(())
        }
    };
}

// mimic_db
// define the stores
// mimic_db!(DATA1, 1, DATA2, 2)
#[macro_export]
macro_rules! mimic_db {
    ($($store_name:ident, $memory_id:expr),*) => {
        thread_local! {
            // Define MEMORY_MANAGER thread-locally
            pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
                RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

            // Create and define each store statically and insert into DB
            $(
                pub static $store_name: RefCell<Store> = RefCell::new($memory_id);
            )*

            // Create DB with inserts for all provided stores
            pub static DB: RefCell<Db> = RefCell::new({
                let mut db = Db::new();

                // Insert each store into DB
                $(
                    db.insert(stringify!($store_name), &$store_name);
                )*

                db
            });
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
        ::mimic::export::defer::defer!(::mimic::log!(
            Log::Perf,
            "api call used {} instructions ({})",
            ::mimic::ic::api::performance_counter(1),
            module_path!()
        ));
    };
}
