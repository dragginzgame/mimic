use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

///
/// Timestamp
///

#[derive(
    CandidType,
    Clone,
    Copy,
    Debug,
    Deref,
    DerefMut,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct Timestamp(u64);

impl From<u64> for Timestamp {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl Timestamp {
    #[must_use]
    pub fn now() -> Timestamp {
        Self(lib_time::now())
    }

    #[must_use]
    pub fn now_millis() -> Timestamp {
        Self(lib_time::now_millis())
    }
}
