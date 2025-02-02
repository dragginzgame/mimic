use crate::ThisError;
use candid::CandidType;
use ciborium::{de::from_reader, ser::into_writer, Value};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

///
/// Serialize/Deserialize
/// forces use of cbor (ciborium)
///

///
/// SerializeError
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum SerializeError {
    #[error("serialize error: {0}")]
    Serialize(String),

    #[error("deserialize error: {0}")]
    Deserialize(String),
}

// to_binary
pub fn to_binary<T>(ty: &T) -> Result<Vec<u8>, SerializeError>
where
    T: Serialize,
{
    let mut writer = Vec::<u8>::new();
    into_writer(ty, &mut writer).map_err(|e| SerializeError::Serialize(e.to_string()))?;

    Ok(writer)
}

// from_binary
pub fn from_binary<T>(bytes: &[u8]) -> Result<T, SerializeError>
where
    T: DeserializeOwned,
{
    from_reader(bytes).map_err(|e| {
        // attempt to deserialize into a more generic Value for debugging
        match from_reader::<Value, _>(bytes) {
            Ok(value) => {
                SerializeError::Deserialize(format!("failed to deserialize: {e} ({value:?})"))
            }
            Err(debug_error) => SerializeError::Deserialize(format!(
                "failed to deserialize: {e}. DEBUG FAILED {debug_error}"
            )),
        }
    })
}
