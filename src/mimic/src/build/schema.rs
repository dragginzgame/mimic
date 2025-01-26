use crate::orm::schema::build::BuildError;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// SchemaError
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum SchemaError {
    #[snafu(transparent)]
    BuildError { source: BuildError },
}

// schema
pub fn schema() -> Result<String, SchemaError> {
    let schema = crate::orm::schema::build::get_schema()?;
    let output = serde_json::to_string(&*schema).unwrap();

    Ok(output)
}
