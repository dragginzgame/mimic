use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    // init
    mimic_base::init();

    // cargo directives
    println!("cargo:rerun-if-changed=build.rs");

    // Get the output directory set by Cargo
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    //
    // actor
    //
    let output = build::actor("test").unwrap();

    // Write the output (stdout) to the specified file in OUT_DIR
    let actor_file = PathBuf::from(out_dir.clone()).join("actor.rs");
    let mut file = File::create(actor_file)?;
    file.write_all(output.as_bytes())?;

    //
    // schema
    //
    let output = build::schema().unwrap();

    // Write the output (stdout) to the specified file in OUT_DIR
    let schema_file = PathBuf::from(out_dir).join("schema.rs");
    let mut file = File::create(schema_file)?;
    file.write_all(output.as_bytes())?;

    Ok(())
}
