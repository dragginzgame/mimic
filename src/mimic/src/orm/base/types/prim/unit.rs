use crate::orm::{prelude::*, traits::ValidateAuto};
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

impl Orderable for Unit {}

impl ValidateManual for Unit {}

impl ValidateAuto for Unit {}

impl Visitable for Unit {}
