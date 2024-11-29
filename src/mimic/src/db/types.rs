use crate::ic::structures::{
    serialize::{from_binary, to_binary},
    storable::Bound,
    Storable,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

///
/// STORAGE & API TYPES
///

///
/// DataRow
/// the data B-tree key and value pair
///

#[derive(Clone, Debug)]
pub struct DataRow {
    pub key: DataKey,
    pub value: DataValue,
}

impl DataRow {
    #[must_use]
    pub const fn new(key: DataKey, value: DataValue) -> Self {
        Self { key, value }
    }
}

///
/// DataValue
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct DataValue {
    pub data: Vec<u8>,
    pub path: String,
    pub metadata: Metadata,
}

impl Storable for DataValue {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(to_binary(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        from_binary(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

///
/// DataKey
///

#[derive(CandidType, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct DataKey(Vec<(String, Option<String>)>);

impl DataKey {
    #[must_use]
    pub const fn new(parts: Vec<(String, Option<String>)>) -> Self {
        Self(parts)
    }

    /// Creates an upper bound for the `DataKey` by appending `~` to the last part's key.
    #[must_use]
    pub fn create_upper_bound(&self) -> Self {
        let mut new_parts = self.0.clone();

        if let Some((_, ref mut last_key)) = new_parts.last_mut() {
            match last_key {
                Some(key) => key.push('~'), // Append `~` to the existing key
                None => *last_key = Some("~".to_string()), // Create a new key with `~` if None
            }
        }

        Self(new_parts)
    }
}

impl fmt::Display for DataKey {
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

impl Storable for DataKey {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(to_binary(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        from_binary(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 255,
        is_fixed_size: false,
    };
}

///
/// Metadata
///

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub created: u64,
    pub modified: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_key_order() {
        let parts = vec![
            ("part1".to_string(), Some("alpha".to_string())),
            ("part2".to_string(), Some("gamma".to_string())),
        ];
        let data_key = DataKey::new(parts);
        let upper_bound_key = data_key.create_upper_bound();

        assert!(
            data_key < upper_bound_key,
            "The original key should be less than the upper bound key."
        );
    }

    #[test]
    fn test_empty_last_key() {
        let parts = vec![
            ("part1".to_string(), Some("alpha".to_string())),
            ("part2".to_string(), None), // Initially empty key
        ];
        let data_key = DataKey::new(parts);
        let upper_bound_key = data_key.create_upper_bound();

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
        let data_key = DataKey::new(parts);
        let upper_bound_key = data_key.create_upper_bound();

        assert_eq!(
            upper_bound_key.0.last().unwrap().1.as_deref(),
            Some("gamma~"),
            "The last item should be 'gamma~'."
        );
    }

    #[test]
    fn test_rarity_ordering() {
        let rarity_empty = DataKey::new(vec![("Rarity".to_string(), None)]);
        let rarity_with_key =
            DataKey::new(vec![("Rarity".to_string(), Some("123123".to_string()))]);
        let rarity_upper_bound = DataKey::new(vec![("Rarity".to_string(), Some("~".to_string()))]);

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
