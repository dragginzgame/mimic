use crate::{
    impl_storable_bounded,
    orm::traits::{Filterable, Inner, Orderable, ValidateAuto, ValidateCustom, Visitable},
};
use candid::CandidType;
use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{self},
};

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

    /// Creates an upper bound for the `DataKey` by appending `~` to the last part's key.
    #[must_use]
    pub fn create_upper_bound(&self) -> Self {
        let mut new_parts = self.0.clone();

        if let Some((_, last_key)) = new_parts.last_mut() {
            match last_key {
                Some(key) => key.push('~'),
                None => *last_key = Some("~".to_string()),
            }
        }

        Self(new_parts)
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

impl Filterable for SortKey {}

impl Inner<Self> for SortKey {
    fn inner(&self) -> &Self {
        self
    }

    fn into_inner(self) -> Self {
        self
    }
}

impl Orderable for SortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self, other)
    }
}

impl_storable_bounded!(SortKey, 255, false);

impl ValidateAuto for SortKey {}

impl ValidateCustom for SortKey {}

impl Visitable for SortKey {}

///
/// SortKeySet
///

#[derive(
    CandidType,
    Clone,
    Debug,
    Default,
    Deref,
    DerefMut,
    Eq,
    PartialEq,
    IntoIterator,
    Serialize,
    Deserialize,
)]
pub struct SortKeySet(HashSet<SortKey>);

impl fmt::Display for SortKeySet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self
            .0
            .iter()
            .map(SortKey::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{formatted}]")
    }
}

impl<'a> IntoIterator for &'a SortKeySet {
    type Item = &'a SortKey;
    type IntoIter = std::collections::hash_set::Iter<'a, SortKey>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
