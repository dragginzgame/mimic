#[macro_export]
macro_rules! mimic_build {
    ($actor:expr) => {
        use std::fs::File;
        use std::io::{self, Write};
        use std::path::PathBuf;

        // cargo directives
        println!("cargo:rerun-if-changed=build.rs");

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
