use crate::{
    prelude::*,
    traits::{Inner, ValidateAuto, ValidateCustom},
};
use serde::{Deserialize, Serialize};

///
/// Unit
///

#[derive(
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct Unit();

impl Inner<Self> for Unit {
    fn inner(&self) -> &Self {
        self
    }

    fn into_inner(self) -> Self {
        self
    }
}

impl Orderable for Unit {}

impl Searchable for Unit {}

impl ValidateAuto for Unit {}

impl ValidateCustom for Unit {}

impl Visitable for Unit {}
