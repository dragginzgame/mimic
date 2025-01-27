use crate::{
    core::schema::{get_schema, SchemaError as CoreSchemaError},
    orm::schema::node::{Canister, CanisterBuild},
};
use candid::CandidType;
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
    CoreSchemaError { source: CoreSchemaError },
}

///
/// SCHEMA FUNCTIONS
///

// canisters_by_build
pub fn canisters_by_build(build: CanisterBuild) -> Result<Vec<Canister>, SchemaError> {
    let schema = get_schema()?;
    let canisters: Vec<Canister> = schema
        .filter_nodes::<Canister, _>(|canister| canister.build == build)
        .map(|(_, v)| v)
        .cloned()
        .collect();

    Ok(canisters)
}

// canister
pub fn canister(path: &str) -> Result<Canister, SchemaError> {
    let schema = get_schema()?;
    let canister = schema.get_node::<Canister>(path).cloned().ok_or_else(|| {
        SchemaError::CanisterNotFound {
            path: path.to_string(),
        }
    })?;

    Ok(canister)
}
