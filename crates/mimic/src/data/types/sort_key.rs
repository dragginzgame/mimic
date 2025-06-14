use candid::CandidType;
use icu::impl_storable_bounded;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::{self, Display};

///
/// SortKey
///

#[derive(
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct SortKey(Vec<SortKeyPart>);

impl SortKey {
    #[must_use]
    pub fn new(parts: Vec<(String, Option<String>)>) -> Self {
        let parts = parts
            .into_iter()
            .map(|(path, value)| SortKeyPart::from_path(&path, value))
            .collect();

        Self(parts)
    }

    /// Creates an upper bound by appending '~' to the last value
    #[must_use]
    pub fn create_upper_bound(&self) -> Self {
        let mut parts = self.0.clone();

        if let Some(last) = parts.last_mut() {
            last.value = Some(match &last.value {
                Some(s) => format!("{s}~"),
                None => "~".to_string(),
            });
        }

        Self(parts)
    }
}

impl Display for SortKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts = self
            .0
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{}]", parts)
    }
}

impl_storable_bounded!(SortKey, 128, false);

///
/// SortKeyPart
///

#[derive(
    CandidType, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct SortKeyPart {
    pub path_id: u64,
    pub value: Option<String>,
}

impl SortKeyPart {
    pub fn from_path(path: &str, value: Option<String>) -> Self {
        let path_id = hash_path_to_u64(path);

        Self { path_id, value }
    }
}

impl Display for SortKeyPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(v) => write!(f, "#{} ({})", self.path_id, v),
            None => write!(f, "#{} (None)", self.path_id),
        }
    }
}

///
/// Helper
///

pub fn hash_path_to_u64(path: &str) -> u64 {
    let mut hasher = Sha256::new();

    hasher.update(path.as_bytes());
    let result = hasher.finalize();

    // Truncate the first 8 bytes into a u64 (big endian)
    u64::from_be_bytes([
        result[0], result[1], result[2], result[3], result[4], result[5], result[6], result[7],
    ])
}
