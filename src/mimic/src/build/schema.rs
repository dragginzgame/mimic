use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(transparent)]
    Schema {
        source: crate::orm::schema::build::Error,
    },
}

// schema
pub fn schema() -> Result<String, Error> {
    let output = serde_json::to_string(&*crate::orm::schema::build::schema()?).unwrap();

    Ok(output)
}
