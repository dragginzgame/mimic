pub mod actor;

use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Actor { source: actor::Error },

    #[snafu(transparent)]
    Build {
        source: crate::orm::schema::build::Error,
    },
}

// actor
pub fn actor(canister_name: &str) -> Result<String, Error> {
    let res = actor::generate(canister_name)?;

    Ok(res)
}

// schema
pub fn schema() -> Result<String, Error> {
    let schema = crate::orm::schema::build::schema()?;
    let output = serde_json::to_string(&*schema).unwrap();

    Ok(output)
}
