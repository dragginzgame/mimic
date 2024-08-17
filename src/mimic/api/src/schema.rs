use crate::Error;
use candid::CandidType;
use orm_schema::node::{Canister, CanisterBuild};
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

// canisters_by_build
pub fn canisters_by_build(build: CanisterBuild) -> Result<Vec<Canister>, Error> {
    let schema = core_schema::get_schema().map_err(SchemaError::from)?;
    let canisters: Vec<Canister> = schema
        .filter_nodes::<Canister, _>(|canister| canister.build == build)
        .map(|(_, v)| v)
        .cloned()
        .collect();

    Ok(canisters)
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
