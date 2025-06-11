use crate::types::Key;
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use icu::{impl_storable_bounded, impl_storable_unbounded};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt};

///
/// STORAGE & API TYPES
///

///
/// IndexKey
///

#[derive(
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct IndexKey {
    pub entity: String,
    pub fields: Vec<String>,
    pub values: Vec<String>,
}

impl IndexKey {
    #[must_use]
    pub fn new<S: ToString>(entity: S, fields: &[S], values: &[S]) -> Self {
        Self {
            entity: entity.to_string(),
            fields: fields.iter().map(ToString::to_string).collect(),
            values: values.iter().map(ToString::to_string).collect(),
        }
    }
}

impl std::fmt::Display for IndexKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({} [{}] [{}])",
            self.entity,
            self.fields.join(", "),
            self.values.join(", ")
        )
    }
}

impl_storable_bounded!(IndexKey, 256, false);

///
/// IndexValue
///

#[derive(CandidType, Clone, Debug, Default, Deref, DerefMut, Serialize, Deserialize)]
pub struct IndexValue(pub HashSet<Key>);

impl IndexValue {
    #[must_use]
    pub fn from_key(key: Key) -> Self {
        Self::from(vec![key])
    }
}

impl<S: Into<Key>> From<Vec<S>> for IndexValue {
    fn from(v: Vec<S>) -> Self {
        Self(v.into_iter().map(Into::into).collect())
    }
}

impl_storable_unbounded!(IndexValue);

///
/// SortKey
///

#[derive(
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct SortKey(pub Vec<(String, Option<String>)>);

impl SortKey {
    #[must_use]
    pub const fn new(parts: Vec<(String, Option<String>)>) -> Self {
        Self(parts)
    }

    /// creates an upper bound for the key by appending `~` to the last part's key.
    #[must_use]
    pub fn create_upper_bound(&self) -> Self {
        let mut parts = self.0.clone();

        if let Some((_, val)) = parts.last_mut() {
            *val = Some(match val {
                Some(s) => format!("{s}~"),
                None => "~".to_string(),
            });
        }

        Self(parts)
    }
}

impl fmt::Display for SortKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_parts: Vec<String> = self
            .0
            .iter()
            .map(|(path, key)| match key {
                Some(k) => format!("{path} ({k})"),
                None => format!("{path} (None)"),
            })
            .collect();

        write!(f, "[{}]", formatted_parts.join(", "))
    }
}

impl_storable_bounded!(SortKey, 256, false);

///
/// DataRow
/// the data B-tree key and value pair
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
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

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DataValue {
    pub bytes: Vec<u8>,
    pub path: String,
    pub metadata: Metadata,
}

impl_storable_unbounded!(DataValue);

///
/// Metadata
///

#[derive(CandidType, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Metadata {
    pub created: u64,
    pub modified: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_key_order() {
        let parts = vec![
            ("part1".to_string(), Some("alpha".to_string())),
            ("part2".to_string(), Some("gamma".to_string())),
        ];
        let sort_key = SortKey::new(parts);
        let upper_bound_key = sort_key.create_upper_bound();

        assert!(
            sort_key < upper_bound_key,
            "The original key should be less than the upper bound key."
        );
    }

    #[test]
    fn test_empty_last_key() {
        let parts = vec![
            ("part1".to_string(), Some("alpha".to_string())),
            ("part2".to_string(), None), // Initially empty key
        ];
        let sort_key = SortKey::new(parts);
        let upper_bound_key = sort_key.create_upper_bound();

        assert!(
            upper_bound_key.0.last().unwrap().1.is_some(),
            "The last key should not be None after creating an upper bound."
        );
        assert_eq!(
            upper_bound_key.0.last().unwrap().1.as_deref(),
            Some("~"),
            "The last item in the empty key should be '~'."
        );
    }

    #[test]
    fn test_non_empty_last_key() {
        let parts = vec![
            ("part1".to_string(), Some("alpha".to_string())),
            ("part2".to_string(), Some("gamma".to_string())),
        ];
        let sort_key = SortKey::new(parts);
        let upper_bound_key = sort_key.create_upper_bound();

        assert_eq!(
            upper_bound_key.0.last().unwrap().1.as_deref(),
            Some("gamma~"),
            "The last item should be 'gamma~'."
        );
    }

    #[test]
    fn test_rarity_ordering() {
        let rarity_empty = SortKey::new(vec![("Rarity".to_string(), None)]);
        let rarity_with_key =
            SortKey::new(vec![("Rarity".to_string(), Some("123123".to_string()))]);
        let rarity_upper_bound = SortKey::new(vec![("Rarity".to_string(), Some("~".to_string()))]);

        assert!(
            rarity_empty < rarity_with_key,
            "Rarity(None) should be less than Rarity(Some('123123'))."
        );
        assert!(
            rarity_with_key < rarity_upper_bound,
            "Rarity(Some('123123')) should be less than Rarity(Some('~'))."
        );
    }
}
