pub mod types;

use crate::ic::structures::{btreemap::BTreeMap, DefaultMemory};
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, thread::LocalKey};
use thiserror::Error as ThisError;
use types::{DataKey, DataValue};

///
/// StoreError
/// this also handles errors from macros when looking up stores
///

#[derive(CandidType, Debug, Serialize, Deserialize, ThisError)]
pub enum StoreError {
    #[error("no stores defined")]
    NoStoresDefined,

    #[error("store '{0}' not found")]
    StoreNotFound(String),
}

///
/// Store
///

#[derive(Deref, DerefMut)]
pub struct Store {
    pub data: BTreeMap<DataKey, DataValue>,
}

impl Store {
    // init
    #[must_use]
    pub fn init(memory: DefaultMemory) -> Self {
        Self {
            data: BTreeMap::init(memory),
        }
    }
}

pub type StoreLocal = &'static LocalKey<RefCell<Store>>;
