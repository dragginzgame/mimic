use clap::Parser;
use schema::build::schema;

// Design files needed to generate the schema
// Both design and types should be included as otherwise MACOS builds can fail
// And we also need to call a init() fn in those crates
#[allow(unused_imports)]
use {base::*, design::*};

//
// CLI
//

#[derive(Parser, Debug)]
#[clap(
    name = "Schema Generator",
    version = "1.0",
    about = "Generates schema JSON"
)]
struct Cli {}

// main
fn main() {
    let _cli = Cli::parse();

    // Stub functions for Rust on OSX
    design::init();
    base::init();

    // validate schema
    if let Err(e) = ::schema::build::validate() {
        eprintln!("{e}");
        std::process::exit(2);
    }

    let output = serde_json::to_string(&*schema()).unwrap();

    println!("{output}");
}
