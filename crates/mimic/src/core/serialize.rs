use serde::{Serialize, de::DeserializeOwned};
use std::cell::Cell;
use std::thread_local;
use thiserror::Error as ThisError;

///
/// SerializeError
///

#[derive(Debug, ThisError)]
pub enum SerializeError {
    #[error(transparent)]
    SerializeError(#[from] icu::serialize::SerializeError),
}

//
// Instrumentation counters for debugging serialization hot paths.
//

thread_local! {
    static SERIALIZE_CALLS: Cell<usize> = const { Cell::new(0) };
    static DESERIALIZE_CALLS: Cell<usize> = const { Cell::new(0) };
}

/// Return current serialize() call count.
#[must_use]
pub fn serialize_call_count() -> usize {
    SERIALIZE_CALLS.with(Cell::get)
}

/// Return current deserialize() call count.
#[must_use]
pub fn deserialize_call_count() -> usize {
    DESERIALIZE_CALLS.with(Cell::get)
}

/// Reset both serialize/deserialize call counters to zero.
pub fn reset_serialize_counters() {
    SERIALIZE_CALLS.with(|c| c.set(0));
    DESERIALIZE_CALLS.with(|c| c.set(0));
}

// serialize
// passes through to the icu default serializer for efficiency
pub fn serialize<T>(ty: &T) -> Result<Vec<u8>, SerializeError>
where
    T: Serialize,
{
    SERIALIZE_CALLS.with(|c| c.set(c.get() + 1));
    icu::serialize::serialize(ty).map_err(SerializeError::from)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, SerializeError>
where
    T: DeserializeOwned,
{
    DESERIALIZE_CALLS.with(|c| c.set(c.get() + 1));
    icu::serialize::deserialize(bytes).map_err(SerializeError::from)
}
