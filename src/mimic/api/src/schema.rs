use crate::Error;
use candid::CandidType;
use orm_schema::node::Canister;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// SchemaError
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum SchemaError {
    #[snafu(display("canister not found in schema: {path}"))]
    CanisterNotFound { path: String },

    #[snafu(transparent)]
    Schema { source: core_schema::Error },
}

///
/// SCHEMA FUNCTIONS
///

// as_json
pub fn as_json() -> Result<String, Error> {
    let json = core_schema::get_schema_json().map_err(SchemaError::from)?;

    Ok(json)
}

// canister
pub fn canister(path: &str) -> Result<Canister, Error> {
    let schema = core_schema::get_schema().map_err(SchemaError::from)?;

    let canister =
        schema
            .get_node::<Canister>(path)
            .cloned()
            .ok_or(SchemaError::CanisterNotFound {
                path: path.to_string(),
            })?;

    Ok(canister)
}
