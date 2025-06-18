use crate::db::types::SortKey;
use candid::{CandidType, Decode, Encode};
use icu::ic::structures::{Storable, storable::Bound};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

///
/// DataRow
/// the data B-tree key and value pair
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct DataRow {
    pub key: SortKey,
    pub value: DataValue,
}

impl DataRow {
    #[must_use]
    pub const fn new(key: SortKey, value: DataValue) -> Self {
        Self { key, value }
    }
}

///
/// DataValue
///
/// custom implementation of Storable because all data goes through this
/// point and we need maximum efficiency
///

#[derive(CandidType, Clone, Debug, Deserialize, Serialize)]
pub struct DataValue {
    pub bytes: Vec<u8>,
    pub path: String,
    pub metadata: Metadata,
}

impl Storable for DataValue {
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

        DataValue {
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
