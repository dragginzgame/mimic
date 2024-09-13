use super::CANISTER_STATE;
use candid::{CandidType, Principal};
use derive_more::{Deref, DerefMut};
use lib_ic::structures::{memory::VirtualMemory, Cell};
use mimic_derive::Storable;
use serde::{Deserialize, Serialize};
use snafu::Snafu;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("path has not been set"))]
    PathNotSet,

    #[snafu(display("root_id has not been set"))]
    RootIdNotSet,

    #[snafu(transparent)]
    Cell {
        source: lib_ic::structures::cell::Error,
    },
}

///
/// CanisterStateManager
///

pub struct CanisterStateManager {}

impl CanisterStateManager {
    // get
    #[must_use]
    pub fn get() -> CanisterState {
        CANISTER_STATE.with_borrow(|state| state.get())
    }

    // set
    pub fn set(new_state: CanisterState) -> Result<(), Error> {
        CANISTER_STATE
            .with_borrow_mut(|state| state.set(new_state))
            .map_err(Error::from)?;

        Ok(())
    }

    // get_path
    pub fn get_path() -> Result<String, Error> {
        let path = Self::get().path.ok_or(Error::PathNotSet)?;

        Ok(path)
    }

    // set_path
    pub fn set_path(canister_type: String) -> Result<(), Error> {
        let mut state = Self::get();
        state.path = Some(canister_type);

        Self::set(state)
    }

    // get_root_id
    pub fn get_root_id() -> Result<Principal, Error> {
        let root_id = Self::get().root_id.ok_or(Error::RootIdNotSet)?;

        Ok(root_id)
    }

    // set_root_id
    pub fn set_root_id(id: Principal) -> Result<(), Error> {
        let mut state = Self::get();
        state.root_id = Some(id);

        Self::set(state)
    }

    // get_parent_id
    #[must_use]
    pub fn get_parent_id() -> Option<Principal> {
        Self::get().parent_id
    }

    // set_parent_id
    pub fn set_parent_id(id: Principal) -> Result<(), Error> {
        let mut state = Self::get();
        state.parent_id = Some(id);

        Self::set(state)
    }
}

///
/// CanisterStateCell
///

#[derive(Deref, DerefMut)]
pub(crate) struct CanisterStateStable(Cell<CanisterState>);

impl CanisterStateStable {
    #[must_use]
    pub fn init(memory: VirtualMemory) -> Self {
        Self(Cell::init(memory, CanisterState::default()).unwrap())
    }
}

///
/// CanisterState
///

#[derive(CandidType, Clone, Debug, Default, Serialize, Deserialize, Storable)]
pub struct CanisterState {
    path: Option<String>,
    root_id: Option<Principal>,
    parent_id: Option<Principal>,
}
