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
    let schema = crate::orm::schema::build::get_schema()?;
    let output = serde_json::to_string(&*schema).unwrap();

    Ok(output)
}
