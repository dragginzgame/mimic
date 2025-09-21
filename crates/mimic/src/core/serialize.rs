use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error as ThisError;

///
/// SerializeError
///

#[derive(Debug, ThisError)]
pub enum SerializeError {
    #[error(transparent)]
    SerializeError(#[from] icu::utils::cbor::SerializeError),
}

//

// serialize
// passes through to the icu default serializer for efficiency
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, SerializeError>
where
    T: Serialize,
{
    icu::utils::cbor::serialize(ty).map_err(SerializeError::from)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, SerializeError>
where
    T: DeserializeOwned,
{
    icu::utils::cbor::deserialize(bytes).map_err(SerializeError::from)
}
