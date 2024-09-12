use super::CHILD_INDEX;
use candid::{CandidType, Principal};
use derive_more::{Deref, DerefMut};
use ic::structures::{memory::VirtualMemory, BTreeMap};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(CandidType, Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("canister not found: {id}"))]
    CanisterNotFound { id: Principal },
}

///
/// ChildIndexManager
///

pub struct ChildIndexManager {}

impl ChildIndexManager {
    // get
    #[must_use]
    pub fn get() -> ChildIndex {
        CHILD_INDEX.with_borrow(|index| index.iter().collect())
    }

    // add_canister
    pub fn add_canister(id: Principal, path: &str) {
        let path = path.to_string();

        CHILD_INDEX.with_borrow_mut(|index| index.insert(id, path));
    }

    // get_canister
    #[must_use]
    pub fn get_canister(id: Principal) -> Option<String> {
        CHILD_INDEX.with_borrow(|index| index.get(&id))
    }

    // try_get_canister
    pub fn try_get_canister(id: Principal) -> Result<String, crate::Error> {
        let canister = Self::get_canister(id).ok_or(Error::CanisterNotFound { id })?;

        Ok(canister)
    }
}

///
/// ChildIndex
/// a map of Child Principal to Canister
///

pub type ChildIndex = Vec<(Principal, String)>;

///
/// ChildIndexStable
///

#[derive(Deref, DerefMut)]
pub struct ChildIndexStable {
    state: BTreeMap<Principal, String>,
}

impl ChildIndexStable {
    // init
    #[must_use]
    pub fn init(memory: VirtualMemory) -> Self {
        Self {
            state: BTreeMap::init(memory),
        }
    }
}
