use derive_more::{Deref, DerefMut};
use mimic::orm::{
    prelude::*,
    traits::{SanitizeAuto, ValidateAuto},
};

///
/// Timestamp
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
pub struct Timestamp(u64);

impl Orderable for Timestamp {}

impl Sanitize for Timestamp {}

impl SanitizeAuto for Timestamp {}

impl Validate for Timestamp {}

impl ValidateAuto for Timestamp {}

impl Visitable for Timestamp {}
