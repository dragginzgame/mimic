// mimic_start
// macro to be included at the start of each canister lib.rs file
#[macro_export]
macro_rules! mimic_start {
    () => {
        // actor.rs
        include!(concat!(env!("OUT_DIR"), "/actor.rs"));

        fn mimic_init() {}
    };
}

// mimic_build
// for the various build.rs files
#[macro_export]
macro_rules! mimic_build {
    ($actor:expr) => {
        use std::{env::var, fs::File, io::Write, path::PathBuf};

        //
        // CARGO
        //
        // should include the build flags we need to get
        // different targets working
        //

        // all
        println!("cargo:rerun-if-changed=build.rs");

        // Get the output directory set by Cargo
        let out_dir = var("OUT_DIR").expect("OUT_DIR not set");

        //
        // ACTOR CODE
        //

        let output = ::mimic::build::generate($actor);

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
