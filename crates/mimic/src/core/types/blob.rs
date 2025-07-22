use crate::core::traits::{
    FieldSearchable, FieldSortable, FieldValue, TypeView, ValidateAuto, ValidateCustom, Visitable,
};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::fmt::{self, Display};

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

impl Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[blob ({} bytes)]", self.0.len())
    }
}

impl FieldSearchable for Blob {}

impl FieldSortable for Blob {}

impl FieldValue for Blob {}

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

impl TypeView for Blob {
    type View = Self;

    fn to_view(&self) -> Self::View {
        self.clone()
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

impl ValidateAuto for Blob {}

impl ValidateCustom for Blob {}

impl Visitable for Blob {}
