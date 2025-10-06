use crate::{Error, core::CoreError};
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error as ThisError;

///
/// SerializeError
///

#[derive(Debug, ThisError)]
pub enum SerializeError {
    #[error(transparent)]
    SerializeError(#[from] canic::utils::cbor::SerializeError),
}

impl From<SerializeError> for Error {
    fn from(err: SerializeError) -> Self {
        CoreError::from(err).into()
    }
}

// serialize
// passes through to the icu default serializer for efficiency
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize,
{
    canic::utils::cbor::serialize(ty)
        .map_err(SerializeError::from)
        .map_err(Error::from)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    canic::utils::cbor::deserialize(bytes)
        .map_err(SerializeError::from)
        .map_err(Error::from)
}
