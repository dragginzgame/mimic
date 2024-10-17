use super::SUBNET_INDEX;
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
    #[snafu(display("canister type not found: {path}"))]
    CanisterTypeNotFound { path: String },
}

///
/// SubnetIndexManager
///

pub struct SubnetIndexManager {}

impl SubnetIndexManager {
    // get
    #[must_use]
    pub fn get() -> SubnetIndex {
        SUBNET_INDEX.with_borrow(|index| index.iter().collect())
    }

    // set
    pub fn set(new_state: SubnetIndex) {
        SUBNET_INDEX.with_borrow_mut(|state| {
            state.clear();
            for (k, v) in new_state {
                state.insert(k, v);
            }
        });
    }

    // get_canister
    #[must_use]
    pub fn get_canister(path: &str) -> Option<Principal> {
        SUBNET_INDEX.with_borrow(|index| index.get(&path.to_string()))
    }

    // try_get_canister
    pub fn try_get_canister(path: &str) -> Result<Principal, Error> {
        let canister = Self::get_canister(path).ok_or_else(|| Error::CanisterTypeNotFound {
            path: path.to_string(),
        })?;

        Ok(canister)
    }

    // set_canister
    pub fn set_canister(path: &str, id: Principal) {
        SUBNET_INDEX.with_borrow_mut(|index| index.insert(path.to_string(), id));
    }
}

///
/// SubnetIndex
/// a map of canister::path String to Canister
///

pub type SubnetIndex = Vec<(String, Principal)>;

#[derive(Deref, DerefMut)]
pub struct SubnetIndexStable {
    state: BTreeMap<String, Principal>,
}

impl SubnetIndexStable {
    // init
    #[must_use]
    pub fn init(memory: VirtualMemory) -> Self {
        Self {
            state: BTreeMap::init(memory),
        }
    }
}
