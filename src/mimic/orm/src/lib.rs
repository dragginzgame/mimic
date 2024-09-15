pub mod collections;
pub mod traits;
pub mod types;
pub mod visit;

use ::types::ErrorTree;
use ic::structures::serialize::{from_binary, to_binary};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::Snafu;
use traits::Visitable;
use visit::{perform_visit, perform_visit_mut, SanitizeVisitor, ValidateVisitor};

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("invalid enum hash '{key}'"))]
    InvalidEnumHash { key: u64 },

    #[snafu(display("cannot parse field '{field}'"))]
    ParseField { field: String },

    #[snafu(display("validation failed: {errors}"))]
    Validation { errors: ErrorTree },

    #[snafu(transparent)]
    Serialize {
        source: ic::structures::serialize::Error,
    },
}

impl Error {
    #[must_use]
    pub fn parse_field(field: &str) -> Self {
        Self::ParseField {
            field: field.to_string(),
        }
    }
}

///
/// TYPE FUNCTIONS
/// The primary functions to validate and manipulate types within the ORM
///

// serialize
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    to_binary::<T>(ty).map_err(Error::from)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    from_binary::<T>(bytes).map_err(Error::from)
}

// sanitize
pub fn sanitize(node: &mut dyn Visitable) {
    let mut visitor = SanitizeVisitor::new();

    perform_visit_mut(&mut visitor, node, "");
}

// validate
pub fn validate(node: &dyn Visitable) -> Result<(), Error> {
    let mut visitor = ValidateVisitor::new();
    perform_visit(&mut visitor, node, "");

    visitor
        .errors
        .result()
        .map_err(|errors| Error::Validation { errors })
}
