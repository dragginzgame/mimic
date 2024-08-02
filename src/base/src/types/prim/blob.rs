use derive_more::{Deref, DerefMut};
use mimic::orm::prelude::*;
use mimic::{
    orm::traits::{Inner, SanitizeAuto, ValidateAuto},
    types::Blob as WrappedBlob,
};

///
/// Blob
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
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct Blob(WrappedBlob);

impl Blob {
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Filterable for Blob {}

impl From<Vec<u8>> for Blob {
    fn from(bytes: Vec<u8>) -> Self {
        Self(WrappedBlob::from(bytes))
    }
}

impl Inner<Self> for Blob {
    fn inner(&self) -> &Self {
        self
    }
}

impl Orderable for Blob {}

impl Sanitize for Blob {}

impl SanitizeAuto for Blob {}

impl Validate for Blob {}

impl ValidateAuto for Blob {}

impl Visitable for Blob {}
