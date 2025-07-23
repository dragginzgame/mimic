use crate::{
    core::{
        Key,
        traits::{EntityKind, Storable},
    },
    db::hasher::xx_hash_u64,
    ic::structures::{BTreeMap, DefaultMemory, storable::Bound},
};
use candid::{CandidType, Decode, Encode};
use derive_more::{Deref, DerefMut};
use icu::impl_storable_bounded;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    cell::RefCell,
    fmt::{self, Display},
    thread::LocalKey,
};

///
/// DataStoreLocal
///

pub type DataStoreLocal = &'static LocalKey<RefCell<DataStore>>;

///
/// DataStore
///

#[derive(Deref, DerefMut)]
pub struct DataStore(BTreeMap<DataKey, DataEntry>);

impl DataStore {
    #[must_use]
    pub fn init(memory: DefaultMemory) -> Self {
        Self(BTreeMap::init(memory))
    }
}

///
/// DataRow
/// the data B-tree key and entry pair
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct DataRow {
    pub key: DataKey,
    pub entry: DataEntry,
}

impl DataRow {
    #[must_use]
    pub const fn new(key: DataKey, entry: DataEntry) -> Self {
        Self { key, entry }
    }
}

///
/// DataKey
///

#[derive(
    CandidType, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct DataKey {
    entity_id: u64,
    key: Key,
}

impl DataKey {
    pub const STORABLE_MAX_SIZE: u32 = 72;

    #[must_use]
    pub fn new<E: EntityKind>(key: impl Into<Key>) -> Self {
        Self::from_path(E::PATH, key)
    }

    #[must_use]
    pub fn from_path(path: &str, key: impl Into<Key>) -> Self {
        Self {
            entity_id: xx_hash_u64(path),
            key: key.into(),
        }
    }

    #[must_use]
    pub const fn key(&self) -> Key {
        self.key
    }

    #[must_use]
    pub const fn max_storable() -> Self {
        Self {
            entity_id: u64::MAX,
            key: Key::max_storable(),
        }
    }
}

impl Display for DataKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} ({})", self.entity_id, self.key)
    }
}

impl From<DataKey> for Key {
    fn from(key: DataKey) -> Self {
        key.key()
    }
}

impl_storable_bounded!(DataKey, Self::STORABLE_MAX_SIZE, false);

///
/// DataEntry
///
/// custom implementation of Storable because all data goes through this
/// point and we need maximum efficiency
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct DataEntry {
    pub bytes: Vec<u8>,
    pub metadata: Metadata,
}

impl DataEntry {
    #[must_use]
    pub fn new(bytes: Vec<u8>, metadata: Metadata) -> Self {
        Self { bytes, metadata }
    }

    fn encode_into_vec(&self) -> Vec<u8> {
        let mut out = Vec::new();

        let blob_bytes = self.bytes.to_bytes();
        write_chunk(&mut out, &blob_bytes);

        let meta_bytes = Encode!(&self.metadata).expect("encode metadata");
        write_chunk(&mut out, &meta_bytes);

        out
    }
}

impl Storable for DataEntry {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(self.encode_into_vec())
    }

    fn into_bytes(self) -> Vec<u8> {
        self.encode_into_vec()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let mut cursor = &bytes[..];

        let bytes = read_chunk(&mut cursor);
        let metadata_buf = read_chunk(&mut cursor);
        let metadata = Decode!(&metadata_buf, Metadata).expect("decode metadata");

        Self { bytes, metadata }
    }
}

// read_chunk
fn read_chunk(buf: &mut &[u8]) -> Vec<u8> {
    assert!(buf.len() >= 4, "not enough bytes for length prefix");
    let len = u32::from_le_bytes(buf[..4].try_into().unwrap()) as usize;

    assert!(
        buf.len() >= 4 + len,
        "chunk length {} exceeds buffer size {}",
        len,
        buf.len()
    );

    let val = buf[4..4 + len].to_vec();
    *buf = &buf[4 + len..];

    val
}

// write_chunk
#[allow(clippy::cast_possible_truncation)]
fn write_chunk(buf: &mut Vec<u8>, data: &[u8]) {
    assert!(data.len() <= u32::MAX as usize, "chunk too large");
    let len = data.len() as u32;

    buf.extend(&len.to_le_bytes());
    buf.extend(data);
}

///
/// Metadata
///

#[derive(CandidType, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Metadata {
    pub created: u64,
    pub modified: u64,
}

impl Metadata {
    #[must_use]
    pub fn new(created: u64, modified: u64) -> Self {
        Self { created, modified }
    }
}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Ulid;

    #[test]
    fn data_entry_round_trip_works() {
        let entry = DataEntry::new(vec![1, 2, 3], Metadata::new(1000, 2000));
        let encoded = Storable::to_bytes(&entry);
        let decoded = DataEntry::from_bytes(encoded);

        assert_eq!(entry.bytes, decoded.bytes);
        assert_eq!(entry.metadata, decoded.metadata);
    }

    #[test]
    fn data_key_max_size_is_bounded() {
        let data_key = DataKey::max_storable();
        let size = Storable::to_bytes(&data_key).len();

        println!("max serialized size = {size}");
        assert!(size <= DataKey::STORABLE_MAX_SIZE as usize);
    }

    #[test]
    fn data_keys_with_identical_paths_and_values_are_equal() {
        let k1 = DataKey::from_path("my::Entity", 1);
        let k2 = DataKey::from_path("my::Entity", 1);

        assert_eq!(k1, k2);
    }

    #[test]
    fn data_keys_with_different_paths_are_not_equal() {
        let k1 = DataKey::from_path("a::Entity", Ulid::from_u128(1));
        let k2 = DataKey::from_path("b::Entity", Ulid::from_u128(1));

        assert_ne!(k1, k2);
    }

    #[test]
    fn data_keys_with_different_values_are_not_equal() {
        let k1 = DataKey::from_path("my::Entity", Ulid::from_u128(1));
        let k2 = DataKey::from_path("my::Entity", Ulid::from_u128(2));

        assert_ne!(k1, k2);
    }

    #[test]
    fn data_keys_are_stable_across_invocations() {
        let k1 = DataKey::from_path("stable::Entity", 42);
        let k2 = DataKey::from_path("stable::Entity", 42);

        assert_eq!(k1, k2);
    }
}
