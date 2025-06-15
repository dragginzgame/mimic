use crate::data::types::hash_path_to_u64;
use candid::CandidType;
use icu::impl_storable_bounded;
use serde::{Deserialize, Serialize};
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
            .map(|(path, value)| SortKeyPart::new(&path, value))
            .collect();

        Self(parts)
    }

    #[must_use]
    pub fn from_parts(parts: Vec<SortKeyPart>) -> Self {
        Self(parts)
    }

    // parts
    #[must_use]
    pub fn parts(&self) -> Vec<SortKeyPart> {
        self.0.clone()
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
    #[must_use]
    pub fn new(path: &str, value: Option<String>) -> Self {
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
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_key_upper_bound_is_strictly_greater_than_original() {
        let original = SortKey::new(vec![
            ("category".to_string(), Some("alpha".to_string())),
            ("type".to_string(), Some("gamma".to_string())),
        ]);

        let upper_bound = original.create_upper_bound();

        assert!(
            original < upper_bound,
            "Expected SortKey to be strictly less than its upper bound"
        );
    }

    #[test]
    fn sort_key_none_is_less_than_some() {
        let none_key = SortKey::new(vec![("rarity".to_string(), None)]);
        let some_key = SortKey::new(vec![("rarity".to_string(), Some("common".to_string()))]);

        assert!(
            none_key < some_key,
            "Expected SortKey with None to sort before SortKey with Some value"
        );
    }

    #[test]
    fn sort_key_some_value_sorts_before_tilde_upper_bound() {
        let value_key = SortKey::new(vec![("rarity".to_string(), Some("123123".to_string()))]);
        let tilde_key = SortKey::new(vec![("rarity".to_string(), Some("~".to_string()))]);

        assert!(
            value_key < tilde_key,
            "Expected SortKey with normal value to sort before '~' suffix upper bound"
        );
    }

    #[test]
    fn sort_keys_with_same_paths_and_values_are_equal() {
        let k1 = SortKey::new(vec![("id".to_string(), Some("abc".to_string()))]);
        let k2 = SortKey::new(vec![("id".to_string(), Some("abc".to_string()))]);

        assert_eq!(k1, k2, "SortKeys with same data should be equal");
        assert_eq!(k1.partial_cmp(&k2), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn sort_key_with_more_parts_is_greater() {
        let short = SortKey::new(vec![("type".to_string(), Some("basic".to_string()))]);
        let long = SortKey::new(vec![
            ("type".to_string(), Some("basic".to_string())),
            ("level".to_string(), Some("2".to_string())),
        ]);

        assert!(
            short < long,
            "SortKey with fewer parts should sort before longer one if prefix matches"
        );
    }
}
