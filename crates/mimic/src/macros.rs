// mimic_start
// macro to be included at the start of each canister lib.rs file
#[macro_export]
macro_rules! mimic_start {
    () => {
        // actor.rs
        include!(concat!(env!("OUT_DIR"), "/actor.rs"));

        fn mimic_init() {
            mimic_init_fixtures().unwrap();
        }
    };
}

// mimic_build
// for the various build.rs files
#[macro_export]
macro_rules! mimic_build {
    ($actor:expr) => {
        use std::{fs::File, io::Write, path::PathBuf};

        //
        // CARGO
        //
        // should include the build flags we need to get
        // different targets working
        //

        // Retrieve the target triple from the environment
        let target = std::env::var("TARGET").unwrap();

        // all
        println!("cargo:rerun-if-changed=build.rs");

        // Get the output directory set by Cargo
        let out_dir = ::std::env::var("OUT_DIR").expect("OUT_DIR not set");

        //
        // ACTOR CODE
        //

        let output = ::mimic::build::actor::generate($actor);

        // write the file
        let actor_file = PathBuf::from(out_dir.clone()).join("actor.rs");
        let mut file = File::create(actor_file)?;
        file.write_all(output.as_bytes())?;
    };
}

// db
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! db {
    () => {
        crate::db()
    };
}
