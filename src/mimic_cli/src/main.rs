pub mod actor;
pub mod schema;

use clap::{Parser, Subcommand};

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
    #[clap(name = "actor", about = "generate actor rust code")]
    Actor(actor::Command),

    #[clap(name = "schema", about = "generate the schema JSON")]
    Schema,
}

///
/// Main
///

fn main() {
    run()
}

pub fn run() {
    let cli = Cli::parse();

    // LOAD SCHEMA
    // Stub functions for Rust on OSX
    mimic_base::init();

    // VALIDATE SCHEMA
    if let Err(e) = orm_schema::build::validate() {
        eprintln!("{e}");
        std::process::exit(2);
    }

    // ROUTE COMMAND
    match cli.command {
        Command::Actor(args) => actor::process(args),
        Command::Schema => schema::process(),
    }
}
