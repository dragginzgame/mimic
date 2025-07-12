use crate::{
    core::{Key, traits::EntityKind},
    db::store::DataKey,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

///
/// RangeExpr
///

#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RangeExpr {
    pub start: DataKey,
    pub end: DataKey,
}

impl RangeExpr {
    #[must_use]
    pub const fn new(start: DataKey, end: DataKey) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub fn from_entity<E: EntityKind>() -> Self {
        let start = DataKey::new(E::PATH, Key::MIN);
        let end = DataKey::new(E::PATH, Key::MAX);

        Self::new(start, end)
    }
}
