use crate::core::traits::{
    FieldSortable, FieldValue, Inner, TypeView, ValidateAuto, ValidateCustom, Visitable,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// Unit
///

#[derive(
    CandidType,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
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

impl TypeView for Unit {
    type View = Self;

    fn to_view(&self) -> Self::View {
        self.clone()
    }

    fn from_view(view: Self::View) -> Self {
        view
    }
}

impl ValidateAuto for Unit {}

impl ValidateCustom for Unit {}

impl Visitable for Unit {}
