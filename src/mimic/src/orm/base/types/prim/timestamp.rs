use crate::orm::{
    prelude::*,
    traits::{SanitizeAuto, ValidateAuto},
};
use derive_more::{Deref, DerefMut};

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

impl SanitizeManual for Timestamp {}

impl SanitizeAuto for Timestamp {}

impl ValidateManual for Timestamp {}

impl ValidateAuto for Timestamp {}

impl Visitable for Timestamp {}
