pub mod actor;
pub mod schema;

use clap::{Parser, Subcommand};

#[macro_use]
extern crate quote;

///
/// Cli
///

#[derive(Parser)]
#[clap(name = "MimiCLI", about = "like dfx but not as good")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

///
/// Command
///

#[derive(Subcommand)]
enum Command {
    #[clap(name = "actor", about = "Commands related to actor")]
    Actor(actor::Command),

    #[clap(name = "schema", about = "generate the schema JSON")]
    Schema,
}

///
/// Main
///

fn main() {
    let cli = Cli::parse();

    // LOAD SCHEMA
    // Stub functions for Rust on OSX
    base::init();

    // VALIDATE SCHEMA
    if let Err(e) = ::schema::build::validate() {
        eprintln!("{e}");
        std::process::exit(2);
    }

    // ROUTE COMMAND
    match cli.command {
        Command::Actor(args) => actor::process(args),
        Command::Schema => schema::process(),
    }
}
