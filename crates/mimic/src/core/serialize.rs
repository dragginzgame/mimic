use serde::{Serialize, de::DeserializeOwned};
use std::cell::Cell;
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
    icu::utils::cbor::serialize(ty).map_err(SerializeError::from)
}

// deserialize
pub fn deserialize<T>(bytes: &[u8]) -> Result<T, SerializeError>
where
    T: DeserializeOwned,
{
    DESERIALIZE_CALLS.with(|c| c.set(c.get() + 1));
    icu::utils::cbor::deserialize(bytes).map_err(SerializeError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counters_increment_and_reset() {
        reset_serialize_counters();
        assert_eq!(serialize_call_count(), 0);
        assert_eq!(deserialize_call_count(), 0);

        // Simple values serialize/deserialize
        let buf = serialize(&123u64).expect("serialize u64");
        assert_eq!(serialize_call_count(), 1);
        assert_eq!(deserialize_call_count(), 0);

        let n: u64 = deserialize(&buf).expect("deserialize u64");
        assert_eq!(n, 123);
        assert_eq!(serialize_call_count(), 1);
        assert_eq!(deserialize_call_count(), 1);

        // Another round to ensure counters keep increasing
        let buf2 = serialize(&"hello").expect("serialize str");
        let s: String = deserialize(&buf2).expect("deserialize String");
        assert_eq!(s, "hello");
        assert_eq!(serialize_call_count(), 2);
        assert_eq!(deserialize_call_count(), 2);

        // Reset back to zero
        reset_serialize_counters();
        assert_eq!(serialize_call_count(), 0);
        assert_eq!(deserialize_call_count(), 0);
    }
}
