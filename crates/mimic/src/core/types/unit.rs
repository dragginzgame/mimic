use crate::core::traits::{
    FieldSortable, FieldValue, Inner, ValidateAuto, ValidateCustom, Visitable,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Unit
///

#[derive(
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct Unit();

impl FieldSortable for Unit {}

impl FieldValue for Unit {}

impl Inner for Unit {
    type Primitive = Self;

    fn inner(&self) -> Self {
        Self()
    }

    fn into_inner(self) -> Self {
        self
    }
}

impl ValidateAuto for Unit {}

impl ValidateCustom for Unit {}

impl Visitable for Unit {}
