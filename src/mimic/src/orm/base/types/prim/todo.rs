use crate::orm::{
    prelude::*,
    traits::{Inner, ValidateAuto},
};
use derive_more::{Deref, DerefMut};
use serde_bytes::ByteBuf;

///
/// Todo
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
pub struct Todo(bool);

impl Filterable for Todo {}

impl Orderable for Todo {}

impl ValidateManual for Todo {}

impl ValidateAuto for Todo {}

impl Visitable for Todo {}
