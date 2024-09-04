///
/// MACROS
///

// mimic_build
// for the various build.rs files
#[macro_export]
macro_rules! mimic_build {
    ($actor:expr) => {
        use std::fs::File;
        use std::io::{self, Write};
        use std::path::PathBuf;

        // cargo directives
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rustc-link-arg=\"-Wl,-all_load\"");

        // Get the output directory set by Cargo
        let out_dir = ::std::env::var("OUT_DIR").expect("OUT_DIR not set");

        //
        // actor
        //

        let output = ::mimic::build::actor($actor).unwrap();

        // write the file
        let actor_file = PathBuf::from(out_dir.clone()).join("actor.rs");
        let mut file = File::create(actor_file)?;
        file.write_all(output.as_bytes())?;

        //
        // schema
        //

        let output = ::mimic::build::schema().unwrap();

        // write the file
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
        include!(concat!(env!("OUT_DIR"), "/actor.rs"));

        // startup
        // code called on all canister startups (install, upgrade)
        fn startup() -> Result<(), Error> {
            let schema_json = include_str!(concat!(env!("OUT_DIR"), "/schema.rs"));
            ::mimic::core::schema::init_schema_json(schema_json).map_err(::mimic::Error::from)?;

            // config
            let toml = include_str!($config);
            ::mimic::config::init_config_toml(toml).map_err(::mimic::Error::from)?;

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
