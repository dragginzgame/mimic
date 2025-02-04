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
        let output = ::mimic::schema::build::get_schema_json().unwrap();

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
        // mimic_init
        fn mimic_init() {
            // schema
            let schema_json = include_str!(concat!(env!("OUT_DIR"), "/schema.rs"));
            ::mimic::schema::state::init_schema_json(schema_json).unwrap();

            // config
            let toml = include_str!($config);
            ::mimic::config::init_config_toml(toml).unwrap();
        }
    };
}

// mimic_memory_manager
#[macro_export]
macro_rules! mimic_memory_manager {
    ($ident:ident) => {
        thread_local! {

            ///
            /// Define MEMORY_MANAGER thread-locally for the entire scope
            ///
            pub static $ident: ::std::cell::RefCell<
                ::mimic::ic::structures::memory::MemoryManager<
                    ::mimic::ic::structures::DefaultMemoryImpl,
                >,
            > = ::std::cell::RefCell::new(::mimic::ic::structures::memory::MemoryManager::init(
                ::mimic::ic::structures::DefaultMemoryImpl::default(),
            ));

        }
    };
}

//
// mimic_stores
// define the stores
// mimic_stores!(MEMORY_MANAGER, DATA1, 1, DATA2, 2)
//
#[macro_export]
macro_rules! mimic_stores {
    // This pattern matches when a memory manager, store names, and memory IDs are provided
    ($memory_manager:expr, $($store_name:ident, $memory_id:expr),*) => {
        thread_local! {
            // Create and define each store statically, initializing with the provided memory ID
            $(
                pub static $store_name: ::std::cell::RefCell<::mimic::store::Store> =
                    ::std::cell::RefCell::new(::mimic::store::Store::init(
                        $memory_manager.with(|mm| mm.borrow().get(
                            ::mimic::ic::structures::memory_manager::MemoryId::new($memory_id)
                        ))
                    ));
            )*
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

#[macro_export]
macro_rules! impl_storable_bounded {
    ($ident:ident, $max_size:expr, $is_fixed_size:expr) => {
        impl Storable for $ident {
            fn to_bytes(&self) -> ::std::borrow::Cow<[u8]> {
                ::std::borrow::Cow::Owned(::mimic::ic::serialize(self).unwrap())
            }

            fn from_bytes(bytes: ::std::borrow::Cow<[u8]>) -> Self {
                ::mimic::ic::deserialize(&bytes).unwrap()
            }

            const BOUND: ::mimic::ic::structures::storable::Bound =
                ::mimic::ic::structures::storable::Bound::Bounded {
                    max_size: $max_size,
                    is_fixed_size: $is_fixed_size,
                };
        }
    };
}

#[macro_export]
macro_rules! impl_storable_unbounded {
    ($ident:ident) => {
        impl Storable for $ident {
            fn to_bytes(&self) -> ::std::borrow::Cow<[u8]> {
                ::std::borrow::Cow::Owned(::mimic::ic::serialize(self).unwrap())
            }

            fn from_bytes(bytes: ::std::borrow::Cow<[u8]>) -> Self {
                ::mimic::ic::deserialize(&bytes).unwrap()
            }

            const BOUND: ::mimic::ic::structures::storable::Bound =
                ::mimic::ic::structures::storable::Bound::Unbounded;
        }
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
