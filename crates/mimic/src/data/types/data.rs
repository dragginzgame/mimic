use crate::data::types::SortKey;
use candid::CandidType;
use icu::impl_storable_unbounded;
use serde::{Deserialize, Serialize};

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
