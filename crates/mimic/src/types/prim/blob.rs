use crate::{
    prelude::*,
    traits::{FormatSortKey, Inner, ValidateAuto},
};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

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
pub struct Blob(ByteBuf);

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

impl From<Vec<u8>> for Blob {
    fn from(bytes: Vec<u8>) -> Self {
        Self(ByteBuf::from(bytes))
    }
}

impl From<&[u8]> for Blob {
    fn from(bytes: &[u8]) -> Self {
        Self(ByteBuf::from(bytes))
    }
}

impl<const N: usize> From<&[u8; N]> for Blob {
    fn from(bytes: &[u8; N]) -> Self {
        Self(ByteBuf::from(&bytes[..]))
    }
}

impl FormatSortKey for Blob {
    fn format_sort_key(&self) -> Option<String> {
        None
    }
}

impl Inner for Blob {
    type Primitive = Self;

    fn inner(&self) -> Self::Primitive {
        self.clone()
    }

    fn into_inner(self) -> Self::Primitive {
        self
    }
}

impl Orderable for Blob {}

impl Searchable for Blob {}

impl ValidateAuto for Blob {}

impl ValidateCustom for Blob {}

impl Visitable for Blob {}
