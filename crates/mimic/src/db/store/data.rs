use crate::{
    core::{traits::Storable, value::IndexValue},
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
    ops::{Range, RangeInclusive},
    thread::LocalKey,
};

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
/// DataStoreLocal
///

pub type DataStoreLocal = &'static LocalKey<RefCell<DataStore>>;

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
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct DataKey(Vec<DataKeyPart>);

impl DataKey {
    #[must_use]
    pub fn new(parts: Vec<(&str, Option<IndexValue>)>) -> Self {
        let parts = parts
            .into_iter()
            .map(|(path, val)| DataKeyPart::new(path, val))
            .collect();

        Self(parts)
    }

    // parts
    #[must_use]
    pub fn parts(&self) -> Vec<DataKeyPart> {
        self.0.clone()
    }
}

impl Display for DataKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts = self
            .0
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{parts}]")
    }
}

impl From<Vec<DataKeyPart>> for DataKey {
    fn from(parts: Vec<DataKeyPart>) -> Self {
        Self(parts)
    }
}

impl_storable_bounded!(DataKey, 128, false);

///
/// DataKeyRange
///

#[derive(Debug)]
pub enum DataKeyRange {
    Inclusive(RangeInclusive<DataKey>),
    Exclusive(Range<DataKey>),
    SkipFirstInclusive(RangeInclusive<DataKey>),
    SkipFirstExclusive(Range<DataKey>),
}

///
/// DataKeyPart
///

#[derive(
    CandidType, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct DataKeyPart {
    pub entity_id: u64,
    pub value: Option<IndexValue>,
}

impl DataKeyPart {
    #[must_use]
    pub fn new(path: &str, value: Option<IndexValue>) -> Self {
        Self {
            entity_id: xx_hash_u64(path),
            value,
        }
    }
}

impl Display for DataKeyPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(v) => write!(f, "#{} ({})", self.entity_id, v),
            None => write!(f, "#{} (None)", self.entity_id),
        }
    }
}

///
/// DataEntry
///
/// custom implementation of Storable because all data goes through this
/// point and we need maximum efficiency
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct DataEntry {
    pub bytes: Vec<u8>,
    pub path: String,
    pub metadata: Metadata,
}

impl Storable for DataEntry {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        let mut out = Vec::new();

        // write blob
        let blob_bytes = self.bytes.to_bytes();
        write_chunk(&mut out, &blob_bytes);

        // write path
        write_chunk(&mut out, self.path.as_bytes());

        // write metadata
        let meta_bytes = Encode!(&self.metadata).expect("encode metadata");
        write_chunk(&mut out, &meta_bytes);

        Cow::Owned(out)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let mut cursor = &bytes[..];

        let bytes = read_chunk(&mut cursor);
        let path = String::from_utf8(read_chunk(&mut cursor)).expect("invalid utf-8 path");
        let metadata_buf = read_chunk(&mut cursor);
        let metadata = Decode!(&metadata_buf, Metadata).expect("decode metadata");

        Self {
            bytes,
            path,
            metadata,
        }
    }
}

// read_chunk
fn read_chunk(buf: &mut &[u8]) -> Vec<u8> {
    let len = u32::from_le_bytes(buf[..4].try_into().unwrap()) as usize;
    let val = buf[4..4 + len].to_vec();
    *buf = &buf[4 + len..];

    val
}

// write_chunk
#[allow(clippy::cast_possible_truncation)]
fn write_chunk(buf: &mut Vec<u8>, data: &[u8]) {
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

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Ulid;

    fn ulid(v: u128) -> IndexValue {
        IndexValue::Ulid(Ulid::from_u128(v))
    }

    fn int(n: i64) -> IndexValue {
        IndexValue::Int(n)
    }

    #[test]
    fn data_keys_with_identical_paths_and_values_are_equal() {
        let k1 = DataKey::new(vec![("my::Entity", Some(ulid(1)))]);
        let k2 = DataKey::new(vec![("my::Entity", Some(ulid(1)))]);

        assert_eq!(k1, k2);
    }

    #[test]
    fn data_keys_with_different_paths_are_not_equal() {
        let k1 = DataKey::new(vec![("a::Entity", Some(ulid(1)))]);
        let k2 = DataKey::new(vec![("b::Entity", Some(ulid(1)))]);

        assert_ne!(k1, k2);
    }

    #[test]
    fn data_keys_with_different_values_are_not_equal() {
        let k1 = DataKey::new(vec![("my::Entity", Some(ulid(1)))]);
        let k2 = DataKey::new(vec![("my::Entity", Some(ulid(2)))]);

        assert_ne!(k1, k2);
    }

    #[test]
    fn data_keys_with_none_and_some_are_different() {
        let k1 = DataKey::new(vec![("my::Entity", None)]);
        let k2 = DataKey::new(vec![("my::Entity", Some(ulid(1)))]);

        assert_ne!(k1, k2);
    }

    #[test]
    fn data_keys_with_additional_parts_are_different() {
        let short = DataKey::new(vec![("my::Entity", Some(ulid(1)))]);
        let long = DataKey::new(vec![
            ("my::Entity", Some(ulid(1))),
            ("my::Entity", Some(ulid(2))),
        ]);

        assert_ne!(short, long);
    }

    #[test]
    fn data_keys_are_stable_across_invocations() {
        let k1 = DataKey::new(vec![("stable::Entity", Some(int(42)))]);
        let k2 = DataKey::new(vec![("stable::Entity", Some(int(42)))]);

        assert_eq!(k1, k2);
    }

    #[test]
    fn data_key_ordering_is_structural_only() {
        let k1 = DataKey::new(vec![
            ("x::Entity", Some(ulid(1))),
            ("y::Entity", Some(ulid(1))),
        ]);
        let k2 = DataKey::new(vec![
            ("x::Entity", Some(ulid(1))),
            ("y::Entity", Some(ulid(2))),
        ]);

        assert_ne!(k1, k2);
    }
}
