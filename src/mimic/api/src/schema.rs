use candid::CandidType;
use core_schema::SCHEMA;
use schema::node::Canister;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// SchemaError
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum SchemaError {
    #[snafu(display("canister not found in schema: {path}"))]
    CanisterNotFound { path: String },
}

///
/// SCHEMA FUNCTIONS
///

// canister
pub fn canister(path: &str) -> Result<Canister, SchemaError> {
    SCHEMA
        .get_node::<Canister>(path)
        .cloned()
        .ok_or(SchemaError::CanisterNotFound {
            path: path.to_string(),
        })
}
