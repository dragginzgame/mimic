use super::APP_STATE;
use candid::CandidType;
use derive_more::{Deref, DerefMut};
use ic::{
    log,
    structures::{
        memory::VirtualMemory,
        serialize::{from_binary, to_binary},
        storable::Bound,
        Cell, Storable,
    },
    Log,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use std::borrow::Cow;
use strum::Display;

///
/// Error
///

#[derive(Debug, Serialize, Deserialize, Snafu)]
pub enum Error {
    #[snafu(display("app is already in {mode} mode"))]
    AlreadyInMode { mode: AppMode },

    #[snafu(transparent)]
    Cell { source: ic::structures::cell::Error },
}

///
/// AppStateManager
///

pub struct AppStateManager {}

impl AppStateManager {
    // get
    #[must_use]
    pub fn get() -> AppState {
        APP_STATE.with_borrow(|state| state.get())
    }

    // set
    pub fn set(new_state: AppState) -> Result<(), Error> {
        APP_STATE
            .with_borrow_mut(|state| state.set(new_state))
            .map_err(Error::from)?;

        Ok(())
    }

    // get_mode
    #[must_use]
    pub fn get_mode() -> AppMode {
        APP_STATE.with_borrow(|state| state.get().mode)
    }

    // set_mode
    pub fn set_mode(mode: AppMode) -> Result<(), Error> {
        APP_STATE
            .with_borrow_mut(|state| {
                let mut cur_state = state.get();

                cur_state.mode = mode;
                state.set(cur_state)
            })
            .map_err(Error::from)?;

        Ok(())
    }

    // command
    pub fn command(cmd: AppCommand) -> Result<(), Error> {
        let old_mode = Self::get_mode();
        let new_mode = match cmd {
            AppCommand::Start => AppMode::Enabled,
            AppCommand::Readonly => AppMode::Readonly,
            AppCommand::Stop => AppMode::Disabled,
        };

        // update mode
        if old_mode == new_mode {
            Err(Error::AlreadyInMode { mode: old_mode })?;
        }
        Self::set_mode(new_mode)?;

        log!(Log::Ok, "app: mode changed {old_mode} -> {new_mode}");

        Ok(())
    }
}

///
/// AppCommand
///

#[derive(CandidType, Clone, Copy, Debug, Display, Eq, PartialEq, Serialize, Deserialize)]
pub enum AppCommand {
    Start,
    Readonly,
    Stop,
}

///
/// AppStateStable
///

#[derive(Deref, DerefMut)]
pub struct AppStateStable(Cell<AppState>);

impl AppStateStable {
    #[must_use]
    pub fn init(memory: VirtualMemory) -> Self {
        Self(Cell::init(memory, AppState::default()).unwrap())
    }
}

///
/// AppState
///

#[derive(CandidType, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AppState {
    mode: AppMode,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: AppMode::Disabled,
        }
    }
}

impl Storable for AppState {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(to_binary(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        from_binary(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

///
/// AppMode
/// used for the query/update guards
/// Eventually we'll have more granularity overall
///

#[derive(CandidType, Clone, Copy, Debug, Display, Eq, PartialEq, Serialize, Deserialize)]
pub enum AppMode {
    Enabled,
    Readonly,
    Disabled,
}
