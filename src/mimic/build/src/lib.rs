pub mod actor;

use orm_schema::build::schema_read;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Schema { source: orm_schema::Error },
}

// actor
pub fn actor(canister_name: &str) -> Result<String, Error> {
    orm_schema::build::validate()?;

    let res = actor::generate(canister_name)?;

    Ok(res)
}

// schema
pub fn schema() -> Result<String, Error> {
    orm_schema::build::validate()?;

    let output = serde_json::to_string(&*schema_read()).unwrap();

    Ok(output)
}
