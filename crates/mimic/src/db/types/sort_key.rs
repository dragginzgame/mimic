use crate::db::hasher::xx_hash_u64;
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
    pub fn new(parts: Vec<(&str, Option<&str>)>) -> Self {
        let parts = parts
            .into_iter()
            .map(|(path, val)| SortKeyPart::new(path, val.map(|s| s.to_string())))
            .collect();

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

impl From<Vec<SortKeyPart>> for SortKey {
    fn from(parts: Vec<SortKeyPart>) -> Self {
        SortKey(parts)
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
    pub fn new(path: &str, value: Option<String>) -> Self {
        Self {
            entity_id: xx_hash_u64(path),
            value,
        }
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

    fn str(s: &str) -> Option<&str> {
        Some(s)
    }

    #[test]
    fn sort_keys_with_identical_paths_and_values_are_equal() {
        let k1 = SortKey::new(vec![("my::Entity", str("abc"))]);
        let k2 = SortKey::new(vec![("my::Entity", str("abc"))]);

        assert_eq!(k1, k2, "SortKeys from same path and value should be equal");
    }

    #[test]
    fn sort_keys_with_different_paths_are_not_equal() {
        let k1 = SortKey::new(vec![("a::Entity", str("abc"))]);
        let k2 = SortKey::new(vec![("b::Entity", str("abc"))]);

        assert_ne!(k1, k2, "Different paths should produce different SortKeys");
    }

    #[test]
    fn sort_keys_with_different_values_are_not_equal() {
        let k1 = SortKey::new(vec![("my::Entity", str("abc"))]);
        let k2 = SortKey::new(vec![("my::Entity", str("def"))]);

        assert_ne!(k1, k2, "Same path with different values should differ");
    }

    #[test]
    fn sort_keys_with_none_and_some_are_different() {
        let k1 = SortKey::new(vec![("my::Entity", None)]);
        let k2 = SortKey::new(vec![("my::Entity", str("value"))]);

        assert_ne!(k1, k2, "None vs Some should differ");
    }

    #[test]
    fn sort_keys_with_additional_parts_are_different() {
        let short = SortKey::new(vec![("my::Entity", str("v1"))]);
        let long = SortKey::new(vec![("my::Entity", str("v1")), ("my::Entity", str("v2"))]);

        assert_ne!(short, long, "Longer SortKey should not equal shorter one");
    }

    #[test]
    fn sort_keys_are_stable_across_invocations() {
        let k1 = SortKey::new(vec![("stable::Entity", str("42"))]);
        let k2 = SortKey::new(vec![("stable::Entity", str("42"))]);

        assert_eq!(k1, k2, "Hashing should be stable across calls");
    }

    #[test]
    fn sort_key_ordering_is_structural_only() {
        let k1 = SortKey::new(vec![("x::Entity", str("a")), ("y::Entity", str("a"))]);
        let k2 = SortKey::new(vec![("x::Entity", str("a")), ("y::Entity", str("b"))]);

        assert!(k1 != k2, "SortKey ordering should reflect value structure");
    }
}
