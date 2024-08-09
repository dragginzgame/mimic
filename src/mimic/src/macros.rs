///
/// MACROS
///

#[macro_export]
macro_rules! mimic_build {
    ($actor:expr) => {
        use std::env;
        use std::fs::File;
        use std::io::{self, Write};
        use std::path::PathBuf;

        // cargo directives
        println!("cargo:rerun-if-changed=build.rs");

        // Get the output directory set by Cargo
        let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

        //
        // actor
        //
        let output = ::mimic::build::actor($actor).unwrap();

        // Write the output (stdout) to the specified file in OUT_DIR
        let actor_file = PathBuf::from(out_dir.clone()).join("actor.rs");
        let mut file = File::create(actor_file)?;
        file.write_all(output.as_bytes())?;

        //
        // schema
        //
        let output = ::mimic::build::schema().unwrap();

        // Write the output (stdout) to the specified file in OUT_DIR
        let schema_file = PathBuf::from(out_dir).join("schema.rs");
        let mut file = File::create(schema_file)?;
        file.write_all(output.as_bytes())?;
    };
}

// mimic_start
// macro to be included at the start of each canister lib.rs file
#[macro_export]
macro_rules! mimic_start {
    () => {
        include!(concat!(env!("OUT_DIR"), "/actor.rs"));

        // startup
        // code called on all canister startups (install, upgrade)
        fn startup() -> Result<(), Error> {
            //let config_str = include_str!("../../../../../config.toml");
            //::mimic::config::init_config_toml(config_str).map_err(::mimic::Error::from)?;

            let schema_json = include_str!(concat!(env!("OUT_DIR"), "/schema.rs"));
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
