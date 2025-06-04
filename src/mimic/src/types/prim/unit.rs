use crate::{
    prelude::*,
    traits::{FormatSortKey, Inner, ValidateAuto, ValidateCustom},
};
use serde::{Deserialize, Serialize};

///
/// Unit
///

#[derive(
    CandidType, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
pub struct Unit();

impl FormatSortKey for Unit {
    fn format_sort_key(&self) -> Option<String> {
        None
    }
}

impl Inner for Unit {
    type Primitive = Self;

    fn inner(&self) -> Self {
        Self()
    }

    fn into_inner(self) -> Self {
        self
    }
}

impl Orderable for Unit {}

impl Path for Unit {
    const IDENT: &'static str = "Unit";
    const PATH: &'static str = "mimic::types::prim::Unit";
}

impl Searchable for Unit {}

impl ValidateAuto for Unit {}

impl ValidateCustom for Unit {}

impl Visitable for Unit {}
