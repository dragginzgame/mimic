pub mod build;
pub mod helper;
pub mod node;
pub mod types;
pub mod visit;

use candid::CandidType;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Build { source: build::BuildError },

    #[snafu(display("serde json error: {msg}"))]
    SerdeJson { msg: String },
}

///
/// Schemable
///
/// Any data structure requires this trait to be part of the ctor structure
/// that populates the Schema
///
pub trait Schemable {
    // schema
    // generates the structure which is passed to the static Schema data structure
    // via the ctor crate
    fn schema(&self) -> TokenStream;
}

// get_schema_json
// to get the built schema via an executable
pub fn get_schema_json() -> Result<String, Error> {
    let schema = build::schema_read();
    let json =
        serde_json::to_string(&*schema).map_err(|e| Error::SerdeJson { msg: e.to_string() })?;

    Ok(json)
}
