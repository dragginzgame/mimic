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
    fn new(parts: Vec<(u64, Option<String>)>) -> SortKey {
        let parts = parts
            .iter()
            .map(|(id, v)| SortKeyPart::new(*id, v.clone()))
            .collect();

        SortKey::from_parts(parts)
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
    pub entity_id: u64,
    pub value: Option<String>,
}

impl SortKeyPart {
    #[must_use]
    pub fn new(entity_id: u64, value: Option<String>) -> Self {
        Self { entity_id, value }
    }
}

impl Display for SortKeyPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(v) => write!(f, "#{} ({})", self.entity_id, v),
            None => write!(f, "#{} (None)", self.entity_id),
        }
    }
}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;

    fn str(s: &str) -> Option<String> {
        Some(s.to_string())
    }

    #[test]
    fn sort_key_entity_id_dominates_ordering() {
        let lower = SortKey::new(vec![(1, str("alpha"))]);
        let higher = SortKey::new(vec![(2, str("alpha"))]);

        assert!(lower < higher, "Lower entity_id should sort before higher");
    }

    #[test]
    fn sort_key_none_is_less_than_some() {
        let none_key = SortKey::new(vec![(42, None)]);
        let some_key = SortKey::new(vec![(42, str("common"))]);

        assert!(none_key < some_key, "None value should sort before Some");
    }

    #[test]
    fn sort_key_some_value_sorts_before_tilde() {
        let value_key = SortKey::new(vec![(42, str("123123"))]);
        let tilde_key = SortKey::new(vec![(42, str("~"))]);

        assert!(
            value_key < tilde_key,
            "Normal value should sort before tilde '~'"
        );
    }

    #[test]
    fn sort_keys_with_same_data_are_equal() {
        let k1 = SortKey::new(vec![(100, str("abc"))]);
        let k2 = SortKey::new(vec![(100, str("abc"))]);

        assert_eq!(k1, k2, "Identical SortKeys should be equal");
        assert_eq!(k1.partial_cmp(&k2), Some(std::cmp::Ordering::Equal));
    }

    #[test]
    fn sort_key_with_more_parts_is_greater() {
        let short = SortKey::new(vec![(999, str("basic"))]);
        let long = SortKey::new(vec![(999, str("basic")), (999, str("2"))]);

        assert!(
            short < long,
            "Shorter key should sort before longer key with same prefix"
        );
    }

    #[test]
    fn sort_key_with_mixed_entity_ids_sorts_correctly() {
        let k1 = SortKey::new(vec![(1, str("a")), (2, str("b"))]);
        let k2 = SortKey::new(vec![(1, str("a")), (2, str("c"))]);

        assert!(k1 < k2, "Later value should break tie");
    }

    #[test]
    fn sort_key_with_different_entity_ids_in_parts_matters() {
        let k1 = SortKey::new(vec![(1, str("x")), (2, str("y"))]);
        let k2 = SortKey::new(vec![(1, str("x")), (3, str("y"))]);

        assert!(
            k1 < k2,
            "Entity ID in second part should influence ordering"
        );
    }
}
