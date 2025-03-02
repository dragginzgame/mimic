use crate::orm::{
    prelude::*,
    traits::{Inner, ValidateAuto, ValidateCustom},
};
use derive_more::{Deref, DerefMut};
use serde_bytes::ByteBuf;

///
/// Unit
///

#[derive(
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct Unit();

impl Filterable for Unit {}

impl Inner<Self> for Unit {
    fn inner(&self) -> &Self {
        self
    }

    fn into_inner(self) -> Self {
        self
    }
}

impl Orderable for Unit {}

impl ValidateAuto for Unit {}

impl ValidateCustom for Unit {}

impl Visitable for Unit {}
